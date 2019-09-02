use crate::math::vector3::Vec3;

#[derive(Debug)]
pub struct Mat3
{
	pub coef:[f64;9]
}

impl Mat3 {
	pub fn new() -> Mat3
	{
		Mat3 {
			coef:[0.0;9]
		}
	}
	pub fn from(coef:&[f64;9]) -> Mat3
	{
		let mut mat = Mat3::new();
		for i in 0..9
		{
			mat.coef[i] = coef[i];
		}
		mat
	}
    pub fn identity() -> Mat3
    { 
		Mat3::from(&[
			1.0, 0.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 1.0,
		])
    }  
	pub fn from_basis(v1:&Vec3,v2:&Vec3,v3:&Vec3) -> Mat3
	{
		Mat3::from(&[
			v1.x, v2.x, v3.x,
			v1.y, v2.y, v3.y,
			v1.z, v2.z, v3.z,
		])
	}
    pub fn clone(&self) -> Mat3 
    { 
    	Mat3::from(&self.coef)
    }

    pub fn add(& self,m:&Mat3) -> Mat3
    {
		let mut out = Mat3::new();
		for i in 0..9
		{
			out.coef[i] = self.coef[i] + m.coef[i];
		}
		out
    }
    pub fn sub(& self,m:&Mat3) -> Mat3
    {
		let mut out = Mat3::new();
		for i in 0..9
		{
			out.coef[i] = self.coef[i] - m.coef[i];
		}
		out
    }
    pub fn div(& self,s:f64) -> Mat3
    {
		let mut out = Mat3::new();
		for i in 0..9
		{
			out.coef[i] = self.coef[i] / s;
		}
		out
    }
    pub fn mul_mat(&self,m:&Mat3) -> Mat3
    {
    	let mut out = Mat3::new();

		for k in 0..3 
		{
			for j in 0..3
			{
				for i in 0..3
				{
					out.coef[3 * j + k] += self.coef[3 * j + i] * m.coef[3 * i + k];
				}
			}
		}
		out
    }    
    pub fn mul_vec3(&self,v:&Vec3) -> Vec3
    {
		let mut v_as_mat = Mat3::new();

		v_as_mat.coef[0] = v.x;
		v_as_mat.coef[3] = v.y;
		v_as_mat.coef[6] = v.z;

		let out_mat = self.mul_mat(&v_as_mat);
		Vec3::new(
			out_mat.coef[0],
			out_mat.coef[3],
			out_mat.coef[6]
		)
    }
    pub fn det(&self) -> f64
    {
    	  self.coef[0]*self.coef[4]*self.coef[8]
		- self.coef[0]*self.coef[5]*self.coef[7]
		- self.coef[1]*self.coef[3]*self.coef[8]
		+ self.coef[1]*self.coef[5]*self.coef[6]
		+ self.coef[2]*self.coef[3]*self.coef[7]
		- self.coef[2]*self.coef[4]*self.coef[6]	
    }

    pub fn inv(&self) -> Option<Mat3>
    {
    	let det = self.det();
    	if det == 0.0 { return None; }
    	let mut inv = Mat3::from(&[
    		self.coef[4]*self.coef[8]-self.coef[5]*self.coef[7],
			self.coef[2]*self.coef[7]-self.coef[1]*self.coef[8],
			self.coef[1]*self.coef[5]-self.coef[2]*self.coef[4],
			self.coef[5]*self.coef[6]-self.coef[3]*self.coef[8],
			self.coef[0]*self.coef[8]-self.coef[2]*self.coef[6],
			self.coef[2]*self.coef[3]-self.coef[0]*self.coef[5],
			self.coef[3]*self.coef[7]-self.coef[4]*self.coef[6],
			self.coef[1]*self.coef[6]-self.coef[0]*self.coef[7],
			self.coef[0]*self.coef[4]-self.coef[1]*self.coef[3]
    	]);
		inv = inv.div(det);
		return Some(inv);
    }
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let m = Mat3::new();
		
		for i in m.coef.iter()
		{
			assert_eq!(0.0,*i);
		}
	}

	#[test]
	fn test_from() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
		let m = Mat3::from(&coef);
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_identity() {
		let coef = [ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ];
		let m = Mat3::identity();
		
		for i in 0..m.coef.len()
		{
			assert_eq!(coef[i],m.coef[i]);
		}
	}

	#[test]
	fn test_clone() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
		let m1 = Mat3::from(&coef);
		let m2 = Mat3::clone(&m1);

		for i in 0..m1.coef.len()
		{
			assert_eq!(m1.coef[i],m2.coef[i]);
		}
	}

	#[test]
	fn test_add() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
		let m1 = Mat3::from(&coef);
		let m2 = Mat3::clone(&m1);

		let added = m1.add(&m2);

		for i in 0..m1.coef.len()
		{
			assert_eq!(added.coef[i],m1.coef[i]+m2.coef[i]);
		}
	}

	#[test]
	fn test_sub() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
		let m1 = Mat3::from(&coef);
		let m2 = Mat3::clone(&m1);

		let added = m1.sub(&m2);

		for i in 0..m1.coef.len()
		{
			assert_eq!(added.coef[i],m1.coef[i]-m2.coef[i]);
		}
	}

	#[test]
	fn test_div() {
		let coef = [0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
		let m = Mat3::from(&coef);

		let d = m.div(0.5);

		for i in 0..m.coef.len()
		{
			assert_eq!(d.coef[i],m.coef[i]/0.5);
		}
	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-multiply-calculator/%5Cbegin%7Bpmatrix%7D1%262%263%5C%5C%202%261%262%5C%5C%203%262%261%5Cend%7Bpmatrix%7D%5E%7B2%7D

	#[test]
	fn test_mul_mat() {
		let coef = [1.0,2.0,3.0,
					2.0,1.0,2.0,
					3.0,2.0,1.0];

		let m1 = Mat3::from(&coef);
		let m2 = Mat3::from(&coef);

		let out = m1.mul_mat(&m2);

		let expected = [14.0,10.0,10.0,
						10.0, 9.0,10.0,
						10.0,10.0,14.0];

		for i in 0..m1.coef.len()
		{
			assert_eq!(out.coef[i],expected[i]);
		}
	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-multiply-calculator/%5Cbegin%7Bpmatrix%7D1%262%263%5C%5C%20%202%261%262%5C%5C%20%203%262%261%5Cend%7Bpmatrix%7D%5Cbegin%7Bpmatrix%7D1%5C%5C%202%5C%5C%203%5Cend%7Bpmatrix%7D

	#[test]
	fn test_mul_vec3() {
		let coef = [1.0,2.0,3.0,
					2.0,1.0,2.0,
					3.0,2.0,1.0];

		let m = Mat3::from(&coef);

		let out = m.mul_vec3(&Vec3::new(1.0,2.0,3.0));

		let expected = &Vec3::new(14.0,10.0,10.0);

		assert_eq!(out.x,expected.x);
		assert_eq!(out.y,expected.y);
		assert_eq!(out.z,expected.z);

	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-determinant-calculator/%5Cdet%20%5Cbegin%7Bpmatrix%7D1%20%26%202%20%26%203%20%5C%5C4%20%26%205%20%26%206%20%5C%5C7%20%26%208%20%26%209%5Cend%7Bpmatrix%7D?or=ex

	#[test]
	fn test_det_null() {
		let coef = [1.0,2.0,3.0,
					4.0,5.0,6.0,
					7.0,8.0,9.0];

		let m = Mat3::from(&coef);

		let det = m.det();
		let expected = 0.0;

		assert_eq!(det,expected);
	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-determinant-calculator/%5Cdet%20%5Cbegin%7Bpmatrix%7D1%20%26%202%20%26%203%20%5C%5C4%20%26%205%20%26%206%20%5C%5C7%20%26%208%20%26%209%5Cend%7Bpmatrix%7D?or=ex

	#[test]
	fn test_det_not_null() {
		let coef = [1.0,2.0,3.0,
					4.0,9.0,6.0,
					7.0,8.0,9.0];

		let m = Mat3::from(&coef);

		let det = m.det();
		let expected = -48.0;

		assert_eq!(det,expected);
	}

	// computation from 
	// https://www.symbolab.com/solver/matrix-determinant-calculator/%5Cdet%20%5Cbegin%7Bpmatrix%7D1%20%26%202%20%26%203%20%5C%5C4%20%26%205%20%26%206%20%5C%5C7%20%26%208%20%26%209%5Cend%7Bpmatrix%7D?or=ex

	#[test]
	fn test_inv_null_det() {
		let coef = [1.0,2.0,3.0,
					4.0,5.0,6.0,
					7.0,8.0,9.0];

		let m = Mat3::from(&coef);

		let out = m.inv();

		for i in 0..m.coef.len()
		{
			assert!(out.is_none());
		}
	}
	// computation from 
	// https://www.symbolab.com/solver/matrix-determinant-calculator/%5Cdet%20%5Cbegin%7Bpmatrix%7D1%20%26%202%20%26%203%20%5C%5C4%20%26%205%20%26%206%20%5C%5C7%20%26%208%20%26%209%5Cend%7Bpmatrix%7D?or=ex

	#[test]
	fn test_inv() {
		let coef = [1.0,2.0,3.0,
					4.0,5.0,6.0,
					7.0,2.0,9.0];

		let m = Mat3::from(&coef);

		let out = m.inv().unwrap();
		println!("{:?}",out );

		let expected = [-11.0/12.0, 1.0/3.0, 1.0/12.0,
						- 1.0/ 6.0, 1.0/3.0,-1.0/ 6.0,
						  3.0/ 4.0,-1.0/3.0, 1.0/12.0];


		println!("{:?}",expected );

		for i in 0..m.coef.len()
		{
			assert_eq!(out.coef[i],expected[i]);
		}
	}

}