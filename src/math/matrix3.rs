use crate::math::vector3::Vec3;

#[derive(Debug)]
pub struct Mat3
{
	pub m11:f64,
	pub m12:f64,
	pub m13:f64,
	pub m21:f64,
	pub m22:f64,
	pub m23:f64,
	pub m31:f64,
	pub m32:f64,
	pub m33:f64,
}

impl Mat3 {
	pub fn new(m11:f64,m12:f64,m13:f64,m21:f64,m22:f64,m23:f64,m31:f64,m32:f64,m33:f64) -> Mat3
	{
		Mat3 {
			m11, m12, m13,
			m21, m22, m23,
			m31, m32, m33
		}
	}
	pub fn fromBasis(v1:&Vec3,v2:Vec3,v3:Vec3) -> Mat3
	{
		Mat3::new(
			v1.x, v2.x, v3.x,
			v1.y, v2.y, v3.y,
			v1.z, v2.z, v3.z,
		)
	}
    pub fn identity() -> Mat3
    { 
    	Mat3::new(
    		1.0, 0.0, 0.0,
    		0.0, 1.0, 0.0,
    		0.0, 0.0, 1.0,
    	)
    }   
    pub fn clone(&self) -> Mat3 
    { 
    	Mat3::new(
    		self.m11, self.m12, self.m13,
    		self.m11, self.m22, self.m23,
    		self.m11, self.m32, self.m33,
    	)
    }
    pub fn add(&mut self,m:&Mat3)
    {
		self.m11 += m.m11;
		self.m12 += m.m12;
		self.m13 += m.m13;
		self.m21 += m.m21;
		self.m22 += m.m22;
		self.m23 += m.m23;
		self.m31 += m.m31;
		self.m32 += m.m32;
		self.m33 += m.m33;
    }
    pub fn sub(&mut self,m:&Mat3)
    {
		self.m11 -= m.m11;
		self.m12 -= m.m12;
		self.m13 -= m.m13;
		self.m21 -= m.m21;
		self.m22 -= m.m22;
		self.m23 -= m.m23;
		self.m31 -= m.m31;
		self.m32 -= m.m32;
		self.m33 -= m.m33;
    }
    pub fn div(&mut self,s:f64)
    {
		self.m11 /= s;
		self.m12 /= s;
		self.m13 /= s;
		self.m21 /= s;
		self.m22 /= s;
		self.m23 /= s;
		self.m31 /= s;
		self.m32 /= s;
		self.m33 /= s;
    }
    pub fn mulMat(&self,m:&Mat3) -> Mat3
    {
    	Mat3::new(
			m.m11*self.m11+m.m21*self.m12+m.m31*self.m13,
			m.m12*self.m11+m.m22*self.m12+m.m32*self.m13,
			m.m13*self.m11+m.m23*self.m12+m.m33*self.m13,
			m.m11*self.m21+m.m21*self.m22+m.m31*self.m23,
			m.m12*self.m21+m.m22*self.m22+m.m32*self.m23,
			m.m13*self.m21+m.m23*self.m22+m.m33*self.m23,
			m.m11*self.m31+m.m21*self.m32+m.m31*self.m33,
			m.m12*self.m31+m.m22*self.m32+m.m32*self.m33,
			m.m13*self.m31+m.m23*self.m32+m.m33*self.m33
		)
    }
    pub fn det(&self) -> f64
    {
    	  self.m11*self.m22*self.m33
		- self.m11*self.m23*self.m32
		- self.m12*self.m21*self.m33
		+ self.m12*self.m23*self.m31
		+ self.m13*self.m21*self.m32
		- self.m13*self.m22*self.m31	
    }

    pub fn inv(&self) -> Option<Mat3>
    {
    	let det = self.det();
    	if det == 0.0 { return None; }
    	let mut inv = Mat3::new(
    		self.m22*self.m33-self.m23*self.m32,
			self.m13*self.m32-self.m12*self.m33,
			self.m12*self.m23-self.m13*self.m22,
			self.m23*self.m31-self.m21*self.m33,
			self.m11*self.m33-self.m13*self.m31,
			self.m13*self.m21-self.m11*self.m23,
			self.m21*self.m32-self.m22*self.m31,
			self.m12*self.m31-self.m11*self.m32,
			self.m11*self.m22-self.m12*self.m21
    	);
		inv.div(det);
		return Some(inv);
    }
    pub fn transpose(&self) -> Mat3
    {
    	Mat3::new(
    		self.m11,
			self.m21,
			self.m31,
			self.m12,
			self.m22,
			self.m32,
			self.m13,
			self.m23,
			self.m33
    	)
    }
}