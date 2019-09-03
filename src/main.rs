mod loader;
mod math;
mod engine;

fn main() 
{   
	let config = loader::from_path("in.yml");
	let (tracer,mesh) = loader::to_engine(&config);

    let pixels = tracer.render(&mesh);
    let w = config.camera.screen.width;
    let h = config.camera.screen.height;

    lodepng::encode32_file("out.png", &pixels,w,h).expect("Cannot write output image");
}