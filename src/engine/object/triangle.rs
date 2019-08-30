use crate::math::vector3::Vec3;
use crate::math::matrix3::Mat3;
use crate::math::Point3;

#[derive(Debug)]
pub struct Triangle{
	p1:Vec3, 
	p2:Vec3, 
	p3:Vec3
}


impl{
	pub fn new(p1:&Vec3,p2:&Vec3,p3:&Vec3) -> Triangle
	{
		Triangle(p1.clone(),p2.clone(),p3.clone())
	}
	pub fn intersect(direction:&Vec3,origine:&Vec3) -> bool
	{
		// bool ret = false;
		// Matrix3 basisChange = Matrix3::fromBasis(m_vertices[0]-fromPoint,m_vertices[1]-fromPoint,m_vertices[2]-fromPoint).inverse();
		// Vector3 vectInNewBasis = basisChange*vect;
		// if(vectInNewBasis.x>=0 && vectInNewBasis.y>=0 && vectInNewBasis.z>=0)
		// {
		// 	lastIntersectionPoint = fromPoint.translate(vect/(vectInNewBasis.x+vectInNewBasis.y+vectInNewBasis.z));
		// 	ret = true;
		// }
		// return ret;
		false
	}
}