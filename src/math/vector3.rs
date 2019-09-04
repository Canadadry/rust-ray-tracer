#[derive(Debug)]
pub struct Vec3
{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vec3 {
	pub fn null() -> Vec3 { Vec3{x:0.0,y:0.0,z:0.0} }
	pub fn new(x:f64,y:f64,z:f64) -> Vec3 { Vec3{x,y,z} }
	pub fn clone(&self) -> Vec3 { Vec3{x:self.x,y:self.y,z:self.z} }
	pub fn add(&self, v: &Vec3) -> Vec3
	{
		Vec3::new(
			self.x + v.x,
			self.y + v.y,
			self.z + v.z
		)
	}
	pub fn sub(&self, v: &Vec3) -> Vec3
	{
		Vec3::new(
			self.x - v.x,
			self.y - v.y,
			self.z - v.z
		)
	}
	pub fn mul(&self, s:f64) -> Vec3
	{
		Vec3::new(
			self.x * s,
			self.y * s,
			self.z * s
		)
	}
	pub fn div(&self, s:f64) -> Vec3
	{
		Vec3::new(
			self.x / s,
			self.y / s,
			self.z / s
		)
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
	pub fn normalize(&self) -> Vec3
	{
		self.div(self.dot(self).sqrt())
	}
	pub fn project_on(&self,v:&Vec3) -> Vec3
	{	
		let mut projection = v.clone();
		let ratio = self.dot(v)/self.dot(self);
		projection = projection.div(ratio);
		projection
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new() {
		let x = 12.32; let y = 567.985; let z = 62.09;
		let v = Vec3::new(x,y,z);
		
		assert_eq!(x,v.x);
		assert_eq!(y,v.y);
		assert_eq!(z,v.z);
	}

	#[test]
	fn test_clone() {
		let x  = 08.98; let y = 74.391; let z = 98.54;
		let v  = Vec3::new(x,y,z);
		let v2 = v.clone();
		
		assert_eq!(v2.x,v.x);
		assert_eq!(v2.y,v.y);
		assert_eq!(v2.z,v.z);
	}

	#[test]
	fn test_add() {
		let x1 = 08.98; let y1 = 74.391 ; let z1 = 98.54;
		let x2 = 12.32; let y2 = 567.985; let z2 = 62.09;
		let v1 = Vec3::new(x1,y1,z1);
		let v2 = Vec3::new(x2,y2,z2);

		let v3 = v1.add(&v2);
		let v4 = v2.add(&v1);
		
		assert_eq!(v3.x,v4.x);
		assert_eq!(v3.y,v4.y);
		assert_eq!(v3.z,v4.z);

		assert_eq!(x1+x2,v4.x);
		assert_eq!(y1+y2,v4.y);
		assert_eq!(z1+z2,v4.z);
	}

	#[test]
	fn test_sub() {
		let x1 = 08.98;let y1 = 74.391 ;let z1 = 98.54;
		let x2 = 12.32;let y2 = 567.985;let z2 = 62.09;
		let v1 = Vec3::new(x1,y1,z1);
		let v2 = Vec3::new(x2,y2,z2);

		let v3 = v1.sub(&v2);
		let v4 = v2.sub(&v1);
		
		assert_eq!(x1-x2,v3.x);
		assert_eq!(y1-y2,v3.y);
		assert_eq!(z1-z2,v3.z);
		
		assert_eq!(x2-x1,v4.x);
		assert_eq!(y2-y1,v4.y);
		assert_eq!(z2-z1,v4.z);
	}

	#[test]
	fn test_mul() {
		let x = 08.98;let y = 74.391 ;let z = 98.54;
		let s = 12.34; 
		let v = Vec3::new(x,y,z);

		let v2 = v.mul(s);
		
		assert_eq!(x*s,v2.x);
		assert_eq!(y*s,v2.y);
		assert_eq!(z*s,v2.z);
	}

	#[test]
	fn test_div() {
		let x = 08.98;let y = 74.391 ;let z = 98.54;
		let s = 12.34; 
		let v = Vec3::new(x,y,z);

		let v2 = v.div(s);
		
		assert_eq!(x/s,v2.x);
		assert_eq!(y/s,v2.y);
		assert_eq!(z/s,v2.z);
	}

	#[test]
	fn test_div_by_zero() {
		let x = 08.98;let y = 74.391 ;let z = 98.54;
		let s = 0.0; 
		let v = Vec3::new(x,y,z);

		let v2 = v.div(s);
		
		assert_eq!(x/s,v2.x);
		assert_eq!(y/s,v2.y);
		assert_eq!(z/s,v2.z);
	}
	
	// computation done here :
	//    https://www.symbolab.com/solver/vector-dot-product-calculator/%5Cbegin%7Bpmatrix%7D08.98%2674.391%2698.54%5Cend%7Bpmatrix%7D%5Ccdot%5Cbegin%7Bpmatrix%7D12.32%260567.985%2662.09%5Cend%7Bpmatrix%7D
	#[test]
	fn test_dot() {
		let x1 = 08.98;let y1 = 74.391 ;let z1 = 98.54;
		let x2 = 12.32;let y2 = 567.985;let z2 = 62.09;
		let v1 = Vec3::new(x1,y1,z1);
		let v2 = Vec3::new(x2,y2,z2);

		let d1 = v1.dot(&v2);
		let d2 = v2.dot(&v1);

		let dot = 48481.954335;

		assert_eq!(d1,d2);
		assert_eq!(dot,d2);

	}	

	// computation done here :
	//    https://www.symbolab.com/solver/vector-cross-product-calculator/%5Cbegin%7Bpmatrix%7D08.98%2674.391%2698.54%5Cend%7Bpmatrix%7D%5Ctimes%5Cbegin%7Bpmatrix%7D12.32%26567.985%2662.09%5Cend%7Bpmatrix%7D
	#[test]
	fn test_cross() {
		let x1 = 08.98;let y1 = 74.391 ;let z1 = 98.54;
		let x2 = 12.32;let y2 = 567.985;let z2 = 62.09;
		let v1 = Vec3::new(x1,y1,z1);
		let v2 = Vec3::new(x2,y2,z2);

		let c1 = v1.cross(&v2);
		let c2 = v2.cross(&v1);

		let cross = Vec3::new(-51350.30471,656.4446,4184.00818);

		assert_eq!(c1.x,-c2.x);
		assert_eq!(c1.y,-c2.y);
		assert_eq!(c1.z,-c2.z);
		assert!((cross.x-c1.x) < 1e-3f64 );
		assert!((cross.y-c1.y) < 1e-3f64 );
		assert!((cross.z-c1.z) < 1e-3f64 );

	}

	// computation done here :
	//    https://www.symbolab.com/solver/vector-magnitude-calculator/%7C%5Cbegin%7Bpmatrix%7D08.98%2674.391%2698.54%5Cend%7Bpmatrix%7D%7C
	#[test]
	fn test_norm() {
		let x = 08.98;let y = 74.391 ;let z = 98.54;
		let v = Vec3::new(x,y,z);

		let n = v.norm();

		let norm = 123.79334;

		assert!((norm-n) < 1e-3f64 );

	}		

	#[test]
	fn test_project() {
		let x1 = 1.0;let y1 = 1.0 ;let z1 = 1.0;
		let x2 = 0.0;let y2 = 0.0 ;let z2 = 5.0;
		let v1 = Vec3::new(x1,y1,z1);
		let v2 = Vec3::new(x2,y2,z2);

		let p1 = v1.project_on(&v2);
		let p2 = v2.project_on(&v1);

		let projection1 = Vec3::new(0.0,0.0,1.0);
		let projection2 = Vec3::new(1.6,1.6,1.6);

		assert!((projection1.x-p1.x) < 1e-3f64 );
		assert!((projection1.y-p1.y) < 1e-3f64 );
		assert!((projection1.z-p1.z) < 1e-3f64 );

		assert!((projection2.x-p2.x) < 1e-3f64 );
		assert!((projection2.y-p2.y) < 1e-3f64 );
		assert!((projection2.z-p2.z) < 1e-3f64 );

	}


}