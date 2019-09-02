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
	pub fn from_basis(v1:&Vec3,v2:Vec3,v3:Vec3) -> Mat3
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
		v_as_mat.coef[1] = v.y;
		v_as_mat.coef[2] = v.z;

		let out_mat = self.mul_mat(&v_as_mat);
		Vec3::new(
			out_mat.coef[0],
			out_mat.coef[1],
			out_mat.coef[2]
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
}