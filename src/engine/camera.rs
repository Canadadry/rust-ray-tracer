use crate::math::Vec3;
use crate::math::Mat4;

// const W : 16;
// const H : 16;
// const SIZE : W*H;

#[derive(Debug)]
pub struct Camera{
	position:Vec3, 
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