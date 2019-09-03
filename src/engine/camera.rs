use crate::math::vector3::Vec3;
use crate::math::matrix4::Mat4;

pub struct Camera{
	pub position:Vec3, 
	front:Vec3, 
	left:Vec3,
	up:Vec3,
	fov_x:f64
}

impl Camera{
	pub fn new(p:&Vec3,d:&Vec3,u:&Vec3,f:f64) -> Camera
	{
		let mut l = u.cross(&d);
		l = l.div(l.norm());
		let mut u = d.cross(&l);
		u = u.div(u.norm());

		Camera{
			position:p.clone(),
			front:d.clone(),
			left:l.clone(),
			up:u.clone(),
			fov_x:f
		}
	}


	fn build_rotation_table(&self, columns:usize, rows:usize) -> Vec<(f64,f64)>
	{
		let ratio = (columns as f64)/(rows as f64);
		let mut out = Vec::<(f64,f64)>::with_capacity(columns*rows);
		for j in 0..rows
		{
			let j = j as f64;
			let v_rot_angle  = j*self.fov_x/ratio/((rows as f64) -1.0)-self.fov_x/ratio/2.0;

			for i in 0..columns
			{
				let i = i as f64;
				let h_rot_angle  = i*self.fov_x/((columns as f64)-1.0)-self.fov_x/2.0;
				out.push((v_rot_angle,h_rot_angle));
			}
		}
		out
	}

	fn angles_to_ray(&self,v_rot:f64,h_rot:f64) -> Vec3
	{
		let v_rot_matrix = Mat4::rotation(&self.left , v_rot);
		let h_rot_matrix = Mat4::rotation(&self.up   , h_rot);

		let ray = v_rot_matrix.mul_vec3(&self.front);	
		h_rot_matrix.mul_vec3(&ray)
	}

	pub fn get_rays(&self,columns:usize,rows:usize) -> Vec<Vec3>
	{
		let mut ray_list:Vec<Vec3> = Vec::new();
		let angles = self.build_rotation_table(columns,rows);

		for angle in &angles
		{
			ray_list.push(self.angles_to_ray(angle.0,angle.1));
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

		assert_eq!(d.x,cam.front.x);
		assert_eq!(d.y,cam.front.y);
		assert_eq!(d.z,cam.front.z);

		assert_eq!(u.x,cam.up.x);
		assert_eq!(u.y,cam.up.y);
		assert_eq!(u.z,cam.up.z);

		assert_eq!(fov,cam.fov_x);
	}


	#[test]
	fn test_build_rotation_table() {

	}


}