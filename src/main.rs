pub mod math;
pub mod engine;

use engine::tracer::Tracer;
use engine::camera::Camera;
use math::vector3::Vec3;

fn main() {

    let origin    = Vec3::new( 0.0, 0.0, 0.0);
    let look_at   = Vec3::new(10.0,10.0,10.0);
    let direction = look_at.sub(&origin);
    let up        = Vec3::new( 0.0, 0.0, 1.0);
    let fov       = 70.0;
    let screen    = 128usize;

    let tracer    = Tracer::new(Camera::new(&origin,&direction,&up,fov),screen,screen);

    let mesh      = vec![
    					[
    						Vec3::new( 0.0, 0.0, 1.0),
    						Vec3::new( 0.0, 1.0, 1.0),
    						Vec3::new( 1.0, 1.0, 1.0),
    					]
    				]; 

    let pixels = tracer.render(&mesh);

    lodepng::encode32_file("out.png", &pixels, screen,screen).expect("Cannot write output image");
}