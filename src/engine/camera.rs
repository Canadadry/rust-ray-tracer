use crate::math::Vec3;

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
	pub fn getRays(&self) -> [Vec3;0]
	{
		[]
		// rayList.clear();

		// double ratio = (double)column/(double)row;

		// Vector3 left = m_up.crossProduct(m_eyeDirection);
		// left.normalise();
		// Vector3 up = m_eyeDirection.crossProduct(left);
		// up.normalise();

		// Vector3 ray;
		// Matrix4 v_rotation,rotation;
		// rotation.loadIdentity();
		// for(double j = 0; j<row ; j++) //vertical
		// {
		// 	double v_rot = j*m_fovx/ratio/(row-1)-m_fovx/ratio/2.0;
		// 	v_rotation = Matrix4::rotationMatrix(v_rot,left);
		// 	ray = v_rotation*m_eyeDirection;

		// 	for(double i = 0; i<column ; i++) // horizontal
		// 	{
		// 		double h_rot = i*m_fovx/(column-1)-m_fovx/2.0;
		// 		rotation = v_rotation*Matrix4::rotationMatrix(h_rot,up);
		// 		ray = rotation*m_eyeDirection;
		// 		rayList.push_back(ray);
		// 	}
		// }
	}
}