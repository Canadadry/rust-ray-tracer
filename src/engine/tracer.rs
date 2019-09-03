use crate::engine::camera::Camera;
use crate::math::vector3::Vec3;
use crate::math::matrix3::Mat3;


#[derive(Copy, Clone)]
pub struct Pixel(u8,u8,u8,u8);

impl Pixel {
    pub fn white() -> Pixel { Pixel(255,255,255,255)}
    pub fn black() -> Pixel { Pixel(  0,  0,  0,255)}
}
pub struct Tracer{
	pub cam:Camera,
	pub screen:(usize,usize)
}

impl Tracer{

	pub fn new(cam:Camera,width:usize,height:usize) -> Tracer
	{
		Tracer{
			cam,
			screen:(width,height)
		}
	}

	fn triangle_intersect(triangle:&[Vec3;3],origin:&Vec3,ray:&Vec3) -> bool
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

	pub fn render(&self,mesh:&Vec<[Vec3;3]>) -> Vec<Pixel>
	{
		let rays = self.cam.get_rays(self.screen.0,self.screen.1);
		let mut out:Vec<Pixel> = Vec::<Pixel>::with_capacity(self.screen.0*self.screen.1);

		for j in 0..self.screen.1
		{
			for i in 0..self.screen.0
			{
				let mut pixel = Pixel::black();
				for triangle in mesh
				{
					if Tracer::triangle_intersect(triangle,&self.cam.position,&rays[j*self.screen.0+i])
					{
						pixel = Pixel::white();
					}
				}
				out.push(pixel);
			}		
		}
		out
	}
}

