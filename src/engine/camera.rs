use crate::math::vector3::Vec3;
use crate::math::matrix4::Mat4;

// const W : 16;
// const H : 16;
// const SIZE : W*H;

#[derive(Debug)]
pub struct Camera{
	pub position:Vec3, 
	direction:Vec3, 
	up:Vec3,
	fov_x:f64
}

impl Camera{
	pub fn new(p:&Vec3,d:&Vec3,u:&Vec3,f:f64) -> Camera
	{
		Camera{
			position:p.clone(),
			direction:d.clone(),
			up:u.clone(),
			fov_x:f
		}
	}
	pub fn get_rays(&self,columns:u32,rows:u32) -> Vec<Vec3>
	{
		let mut ray_list:Vec<Vec3> = Vec::new();

		let ratio   = (columns as f64)/(rows as f64);

		let mut left = self.up.cross(&self.direction);
		left = left.div(left.norm());
		let mut up = self.direction.cross(&left);
		up = up.div(up.norm());

		for j in 0..rows
		{
			let j = j as f64;
			let v_rot_angle  = j*self.fov_x/ratio/((rows as f64) -1.0)-self.fov_x/ratio/2.0;
			let v_rot_matrix = Mat4::rotation(&left,v_rot_angle);
			let ray = v_rot_matrix.mul_vec3(&self.direction);

			for i in 0..columns
			{
				let i = i as f64;
				let h_rot_angle  = i*self.fov_x/((columns as f64)-1.0)-self.fov_x/2.0;
				let h_rot_matrix = Mat4::rotation(&up,h_rot_angle);
				ray_list.push(h_rot_matrix.mul_vec3(&ray));
			}
		}
		ray_list
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let p = Vec3::new(0.0,0.0,0.0);
		let d = Vec3::new(1.0,0.0,0.0);
		let u = Vec3::new(0.0,0.0,1.0);
		let fov = 90.0;

		let cam = Camera::new(&p,&d,&u,fov);
		
		assert_eq!(p.x,cam.position.x);
		assert_eq!(p.y,cam.position.y);
		assert_eq!(p.z,cam.position.z);

		assert_eq!(d.x,cam.direction.x);
		assert_eq!(d.y,cam.direction.y);
		assert_eq!(d.z,cam.direction.z);

		assert_eq!(u.x,cam.up.x);
		assert_eq!(u.y,cam.up.y);
		assert_eq!(u.z,cam.up.z);

		assert_eq!(fov,cam.fov_x);
	}
}