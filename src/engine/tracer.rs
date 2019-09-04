use crate::engine::camera::Camera;
use crate::math::vector3::Vec3;
use crate::math::matrix3::Mat3;

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Copy, Clone)]
pub struct Pixel(u8,u8,u8,u8);

impl Pixel {
    pub fn white() -> Pixel { Pixel(255,255,255,255)}
    pub fn black() -> Pixel { Pixel(  0,  0,  0,255)}
    pub fn gray(t:f64)  -> Pixel { 
    	let mut color = 255.0*t  ;
    	if color < 0.0   { color = 0.0; }
    	if color > 255.0 { color = 255.0; }
    	let color:u8 = color as u8;
    	Pixel(color,color,color,255)
    }
    pub fn blue() -> Pixel { Pixel(0,0,255,255)}
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

	fn triangle_distance(triangle:&[Vec3;3],origin:&Vec3,ray:&Vec3) -> Option<Vec3>
	{
		let axe1 = ray.mul(-1.0);
		let axe2 = triangle[1].sub(&triangle[0]);
		let axe3 = triangle[2].sub(&triangle[0]);

		let from_basis = Mat3::from_basis(&axe1,&axe2,&axe3);

		let basis = from_basis.inv();
		return match basis {
			None => None,
			Some(basis) => {
				let vec = origin.sub(&triangle[0]);
				let sol = basis.mul_vec3(&vec);
				Some(sol)
			}
		}
	}

	fn compute_color(triangle:&[Vec3;3]) -> Pixel
	{
		let axe1   = triangle[1].sub(&triangle[0]);
		let axe2   = triangle[2].sub(&triangle[0]);
		let normal = axe1.cross(&axe2).normalize();

		let ligh_direction = Vec3::new(5.0,5.0,-5.0).normalize();

		Pixel::gray(ligh_direction.dot(&normal))
	}

	pub fn render(&self,mesh:&Vec<[Vec3;3]>) -> Vec<Pixel>
	{
		let mut rendered_ray = 0;
    	let total_size = self.screen.0*self.screen.1;

		let pb = ProgressBar::new(total_size as u64);
		pb.set_style(ProgressStyle::default_bar()
			.template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
			.progress_chars("#>-"));

		let rays = self.cam.get_rays(self.screen.0,self.screen.1);
		let mut out:Vec<Pixel> = Vec::<Pixel>::with_capacity(total_size);

		for j in 0..self.screen.1
		{
			for i in 0..self.screen.0
			{
				let mut min = 10.0;
				let mut closest:Option<&[Vec3;3]> = None;

				for triangle in mesh
				{
					if Tracer::triangle_intersect(triangle,&self.cam.position,&rays[j*self.screen.0+i])
					{
						let distance = Tracer::triangle_distance(triangle,&self.cam.position,&rays[j*self.screen.0+i]);
						if let Some(v)= distance {
							if min > v.x 
							{ 
								min = v.x;
								closest = Some(&triangle);
							}
						}
					}
				}
				match closest{
					None           => out.push(Pixel::blue()),
					Some(triangle) => out.push(Tracer::compute_color(triangle))
				}
				
				rendered_ray += 1;
		        pb.set_position(rendered_ray);
			}		
		}
	    pb.finish_with_message("done");
		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// computation can easily be done by hand on a drawing 

	#[test]
	fn test_triangle_distance() {
		let line_point = Vec3::new(0.0,0.0,0.0);
		let line_dir   = Vec3::new(7.0,3.0,1.0);
		let p0         = Vec3::new(7.0,0.0,0.0);
		let p1         = Vec3::new(7.0,5.0,0.0);
		let p2         = Vec3::new(7.0,0.0,3.0);
		let triangle   = [p0,p1,p2];

		let expected   = Vec3::new(1.0,3.0/5.0,1.0/3.0);
		let out        = Tracer::triangle_distance(&triangle,&line_point,&line_dir);

		match out {
		    None      => assert!(false),
		    Some(out) => {
		    	assert_eq!(expected.x,out.x);
		    	assert_eq!(expected.y,out.y);
		    	assert_eq!(expected.z,out.z);
		    }
		}
	}

}


