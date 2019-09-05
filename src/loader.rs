use super::engine::tracer::Tracer;
use super::engine::camera::Camera as EngineCam;
use super::engine::tracer::Pixel;
use super::math::vector3::Vec3;

use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Vertex (f64,f64,f64);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Face (u32,u32,u32);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    vertices: Vec::<Vertex>,
    faces   : Vec::<Face>,
    light_direction : Vertex,
    background_color : (u8,u8,u8,u8)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Screen
{
    pub width: usize,
    pub height:usize
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Camera
{
    position: Vertex,
    look_at: Vertex,
    up: Vertex,
    fov: f64,
    pub screen: Screen,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigData
{
    pub camera:Camera,
    scene:Scene
}

pub fn from_path(path:&str) -> ConfigData
{    
    let mut input = String::from("");
    let mut ifile = File::open(&path).expect("unable to open path");    
    ifile.read_to_string(&mut input).expect("unable to read");
    serde_yaml::from_str(&input).expect("invalid format")
}

pub fn to_engine(config:&ConfigData) -> (Tracer,Vec::<[Vec3;3]>)
{

    let origin    = Vec3::new( config.camera.position.0, config.camera.position.1 , config.camera.position.2 );
    let look_at   = Vec3::new( config.camera.look_at.0,  config.camera.look_at.1 ,  config.camera.look_at.2  );
    let direction = look_at.sub(&origin);
    let up        = Vec3::new( config.camera.up.0, config.camera.up.1 , config.camera.up.2 );
    let fov       = config.camera.fov;

    let tracer    = Tracer{
                        cam:EngineCam::new(&origin,&direction,&up,fov),
                        screen:(
                            config.camera.screen.width,
                            config.camera.screen.height
                        ),
                        light:Vec3::new(
                            config.scene.light_direction.0,
                            config.scene.light_direction.1,
                            config.scene.light_direction.2
                        ),
                        background:Pixel{
                            0:config.scene.background_color.0,
                            1:config.scene.background_color.1,
                            2:config.scene.background_color.2,
                            3:config.scene.background_color.3
                        }
                    };

    let mut mesh  = Vec::<[Vec3;3]>::with_capacity(config.scene.faces.len());

    for face  in &config.scene.faces
    {
        let mut v1 = Vec3::null();
        let mut v2 = Vec3::null();
        let mut v3 = Vec3::null();

        if let Some(v) = config.scene.vertices.get((face.0-1) as usize) {
            v1.x = v.0;
            v1.y = v.1;
            v1.z = v.2;
        } else {
            println!("face {:?} has invalid vertice reference : ignored!",face);
            continue;
        };

        if let Some(v) = config.scene.vertices.get((face.1-1) as usize) {
            v2.x = v.0;
            v2.y = v.1;
            v2.z = v.2;
        } else {
            println!("face {:?} has invalid vertice reference : ignored!",face);
            continue;
        };

        if let Some(v) = config.scene.vertices.get((face.2-1) as usize) {
            v3.x = v.0;
            v3.y = v.1;
            v3.z = v.2;
        } else {
            println!("face {:?} has invalid vertice reference : ignored!",face);
            continue;
        };

        mesh.push([v1,v2,v3]);
    }

    (tracer,mesh)

}