use crate::engine::camera::Camera;
use crate::math::vector3::Vec3;
use crate::math::matrix3::Mat3;


#[derive(Copy, Clone)]
struct Pixel(u8,u8,u8,u8);

impl Pixel {
    pub fn red()   -> Pixel { Pixel(255,  0,  0,255)}
    pub fn black() -> Pixel { Pixel(  0,  0,  0,255)}
}

fn triangleIntersect(triangle:&[Vec3;3],origin:&Vec3,ray:&Vec3) -> bool
{
	let axe1 = triangle[0].sub(origin);
	let axe2 = triangle[1].sub(origin);
	let axe3 = triangle[2].sub(origin);

	let basis = Mat3::from_basis(&axe1,&axe2,&axe3).inv();
	return match basis {
		None => false,
		Some(basis) => {
			let ray = basis.mul_vec3(ray);
			(ray.x >= 0.0 && ray.y >= 0.0 && ray.z >= 0.0)
		}
	}
}

fn render(mesh:&Vec<[Vec3;3]>,camera:&Camera)
{
	let size:usize = 16;
	let rays = camera.get_rays(size as u32,size as u32);
	let pr:f64 = 20.0;
	let mut out:Vec<Pixel> = Vec::new();

	for j in 0usize..size
	{
		for i in 0usize..size
		{
			for triangle in mesh
			{
				if triangleIntersect(triangle,&camera.position,&rays[j*size+i])
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