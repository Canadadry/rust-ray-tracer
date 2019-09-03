use crate::math::vector3::Vec3;

#[derive(Debug)]
pub struct Mat4
{
	pub coef:[f64;16]
}

impl Mat4 {
	pub fn new() -> Mat4
	{
		Mat4 {
			coef:[0.0;16]
		}
	}
	pub fn from(coef:&[f64;16]) -> Mat4
	{
		let mut mat = Mat4::new();
		for i in 0..16
		{
			mat.coef[i] = coef[i];
		}
		mat
	}
    pub fn identity() -> Mat4
    { 
		Mat4::from(&[
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
    }   
    pub fn translation(v:&Vec3) -> Mat4
    {
    	Mat4::from(&[
			1.0, 0.0, 0.0, v.x,
			0.0, 1.0, 0.0, v.y,
			0.0, 0.0, 1.0, v.z,
			0.0, 0.0, 0.0, 1.0
		])	
    }
    pub fn rotation(axe:&Vec3, angle:f64) -> Mat4
    {
    	let angle = angle*std::f64::consts::PI/180.0;
	    let normalize_axe = axe.div(axe.norm());

	    let c = angle.cos();
	    let t = 1.0 - c;
	    let s = angle.sin();
	    let x = normalize_axe.x;
	    let y = normalize_axe.y;
	    let z = normalize_axe.z;

		let mut rotation = Mat4::identity();

		rotation.coef[ 0] = x*x*t + c;
	    rotation.coef[ 1] = x*y*t - z*s;
	    rotation.coef[ 2] = x*z*t + y*s;

	    rotation.coef[ 4] = x*y*t + z*s;
	    rotation.coef[ 5] = y*y*t + c;
	    rotation.coef[ 6] = y*z*t - x*s;

	    rotation.coef[ 8] = x*z*t - y*s;
	    rotation.coef[ 9] = y*z*t + x*s;
	    rotation.coef[10] = z*z*t + c;

	    rotation
    }
    pub fn scale(v:&Vec3) -> Mat4
    {
		Mat4::from(&[
			v.x, 0.0, 0.0, 0.0,
			0.0, v.y, 0.0, 0.0,
			0.0, 0.0, v.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
    }
    pub fn clone(&self) -> Mat4 
    { 
    	Mat4::from(&self.coef)
    }
    pub fn add(&self,m:&Mat4) -> Mat4
    {
    	let mut out = Mat4::new();
		for i in 0..16
		{
			out.coef[i] = self.coef[i] + m.coef[i];
		}
		out
    }
    pub fn mul_mat(&self,m:&Mat4) -> Mat4
    {
    	let mut out = Mat4::new();

		for k in 0..4 
		{
			for j in 0..4
			{
				for i in 0..4
				{
					out.coef[4 * j + k] += self.coef[4 * j + i] * m.coef[4 * i + k];
				}
			}
		}
		out
    }

    pub fn mul_vec3(&self,v:&Vec3) -> Vec3
    {
		let mut v_as_mat = Mat4::new();

		v_as_mat.coef[4 * 0] = v.x;
		v_as_mat.coef[4 * 1] = v.y;
		v_as_mat.coef[4 * 2] = v.z;
		v_as_mat.coef[4 * 3] = 1.0;

		let out_mat = self.mul_mat(&v_as_mat);
		
		Vec3::new(
			out_mat.coef[4 * 0] / out_mat.coef[4 * 3],
			out_mat.coef[4 * 1] / out_mat.coef[4 * 3],
			out_mat.coef[4 * 2] / out_mat.coef[4 * 3]
		)
    }

}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let m = Mat4::new();
		
		for i in m.coef.iter()
		{
			assert_eq!(0.0,*i);
		}
	}

	#[test]
	fn test_from() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0];
		let m = Mat4::from(&coef);
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_identity() {
		let coef = [ 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0 ];
		let m = Mat4::identity();
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_translation() {
		let v    = Vec3::new(1.0,2.0,3.0);
		let coef = [ 1.0, 0.0, 0.0, v.x, 0.0, 1.0, 0.0, v.y, 0.0, 0.0, 1.0, v.z, 0.0, 0.0, 0.0, 1.0 ];
		let m = Mat4::translation(&v);
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_rotation() {
		let coef = [ 1.0, 0.0,  0.0, 0.0,
					 0.0, 0.0, -1.0, 0.0,
					 0.0, 1.0,  0.0, 0.0,
					 0.0, 0.0,  0.0, 1.0 ];

		let axe    = Vec3::new(1.0,0.0,0.0);
		let angle  = 90.0;
		let m = Mat4::rotation(&axe,angle);
		
		println!("{:?}",m);

		for i in 0..m.coef.len()
		{
			assert!((coef[i]-m.coef[i]) < 1e-3f64 );
		}
	}

	#[test]
	fn test_scale() {
		let v    = Vec3::new(1.0,2.0,3.0);
		let coef = [ v.x, 0.0, 0.0, 0.0, 0.0, v.y, 0.0, 0.0, 0.0, 0.0, v.z, 0.0, 0.0, 0.0, 0.0, 1.0 ];
		let m = Mat4::scale(&v);
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_clone() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0];
		let m1 = Mat4::from(&coef);
		let m2 = Mat4::clone(&m1);

		for i in 0..m1.coef.len()
		{
			assert_eq!(m1.coef[i],m2.coef[i]);
		}
	}

	#[test]
	fn test_add() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0,15.0];
		let m1 = Mat4::from(&coef);
		let m2 = Mat4::clone(&m1);

		let added = m1.add(&m2);

		for i in 0..m1.coef.len()
		{
			assert_eq!(added.coef[i],m1.coef[i]+m2.coef[i]);
		}
	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-multiply-calculator/%5Cbegin%7Bpmatrix%7D1%262%263%264%5C%5C%202%261%262%263%5C%5C%203%264%263%262%5C%5C%204%263%262%261%5Cend%7Bpmatrix%7D%5E%7B2%7D	

	#[test]
	fn test_mul_mat() {
		let coef = [1.0,2.0,3.0,4.0,
					2.0,1.0,2.0,3.0,
					3.0,4.0,3.0,2.0,
					4.0,3.0,2.0,1.0];

		let m1 = Mat4::from(&coef);
		let m2 = Mat4::from(&coef);

		let out = m1.mul_mat(&m2);
		println!("{:?}",out );
		let expected = [30.0, 28.0, 24.0, 20.0,
						22.0, 22.0, 20.0, 18.0,
						28.0, 28.0, 30.0, 32.0,
						20.0, 22.0, 26.0, 30.0];

		for i in 0..m1.coef.len()
		{
			assert_eq!(out.coef[i],expected[i]);
		}

	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-multiply-calculator/%5Cbegin%7Bpmatrix%7D1%262%263%264%5C%5C%20%20%202%261%262%263%5C%5C%20%20%203%264%263%262%5C%5C%20%20%204%263%262%261%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D1%5C%5C%20%20%202%5C%5C%20%20%203%5C%5C%20%20%201.0%5Cend%7Bpmatrix%7D
	// dont forget to delete by t : (x,y,z,t) => (x/t,y/t,z/t,1.0)

	#[test]
	fn test_mul_vec3() {
		let coef = [1.0,2.0,3.0,4.0,
					2.0,1.0,2.0,3.0,
					3.0,4.0,3.0,2.0,
					4.0,3.0,2.0,1.0];

		let m = Mat4::from(&coef);

		let out = m.mul_vec3(&Vec3::new(1.0,2.0,3.0));
		println!("{:?}",out );
		let expected = Vec3::new(18.0/17.0,13.0/17.0,22.0/17.0);

		assert_eq!(expected.x,out.x);
		assert_eq!(expected.y,out.y);
		assert_eq!(expected.z,out.z);

	}
}
