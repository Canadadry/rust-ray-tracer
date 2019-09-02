use crate::engine::camera::Camera;


#[derive(Copy, Clone)]
struct Pixel(u8,u8,u8,u8);

impl Pixel {
    pub fn red()   -> Pixel { Pixel(255,  0,  0,255)}
    pub fn bleck() -> Pixel { Pixel(  0,  0,  0,255)}
}

fn triangleIntersect(triangle:&[Vec3;3],origin:&Vec3,ray:&Vec3) -> bool
{
	let axe1 = triangle[0].sub(origin);
	let axe2 = triangle[1].sub(origin);
	let axe3 = triangle[2].sub(origin);

	let basis = Mat3::fromBasis(axe1,axe2,axe3).inverse();
	return match basis {
		None => false,
		Some(basis) => {
			let ray = basis.mulVec3(ray);
			(ray.x >= 0 && ray.y >= 0 && ray.z >= 0);
		}
	}
}

fn render(mesh:&Vec<[Vec3;3]>camera:&Camera)
{
	let size:u32 = 16;
	let rays = camera.getRays(size,size);
	let pr:f64 = 20.0;
	let out:Vec<Pixel> = Vec::new();

	for j in [..size]
	{
		for i in [..size]
		{
			for triangle in mesh
			{
				if triangleIntersect(triangle,rays[j*size+i])
				{
					out.push(Pixel::red());
				}
				else {
					out.push(Pixel::black());
				}
			}
		}		
	}
}