#[derive(Debug)]
pub struct Vec3
{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vec3 {
    pub fn new(x:f64,y:f64,z:f64) -> Vec3 { Vec3{x,y,z} }
    pub fn clone(&self) -> Vec3 { Vec3{x:self.x,y:self.y,z:self.z} }
    pub fn add(&mut self, v: &Vec3) 
    {
		self.x += v.x;
		self.y += v.y;
		self.z += v.z;
 	}
    pub fn sub(&mut self, v: &Vec3) 
    {
		self.x -= v.x;
		self.y -= v.y;
		self.z -= v.z;
 	}
    pub fn mul(&mut self, s:f64) 
    {
		self.x *= s;
		self.y *= s;
		self.z *= s;
 	}
    pub fn div(&mut self, s:f64) 
    {
		self.x /= s;
		self.y /= s;
		self.z /= s;
 	}
    pub fn dot(&self, v: &Vec3) -> f64
    {
		self.x*v.x+self.y*v.y+self.z*v.z
 	}
    pub fn cross(&self, v: &Vec3) -> Vec3
    {
		Vec3::new(	
			self.y*v.z-self.z*v.y,
			self.z*v.x-self.x*v.z,
			self.x*v.y-self.y*v.x
		)
 	}
    pub fn norm(&self) -> f64
    {
    	self.dot(self).sqrt()
 	}
    pub fn normalise(&mut self)
    {
    	self.div(self.norm());
 	}
 	pub fn project_on(&self,v:&Vec3) -> Vec3
 	{	
		let mut projection = v.clone();
		let ratio = self.dot(v)/self.dot(self);
		projection.mul(ratio);
		projection
 	}
}
