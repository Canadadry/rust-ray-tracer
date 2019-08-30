use crate::engine::camera::Camera;

fn render(camera:&Camera)
{
	// 	if(m_scene.size()>0)
	// {
	// 	std::vector<Vector3> rayList;
	// 	camera.populateRayListToCast(rayList,outImage.width,outImage.height);
	// 	double minDist = DBL_MAX;
	// 	Color colorForMinDist(Color::black);
	// 	Object* currentObject;
	// 	char* outImageData = outImage.data();
	// 	for(int j=0;j<outImage.height;j++)
	// 	{
	// 		for(int i=0;i<outImage.width;i++)
	// 		{
	// 			minDist = DBL_MAX;
	// 			for(unsigned int k=0;k<m_scene.size();k++)
	// 			{
	// 				currentObject = m_scene[k];
	// 				if(currentObject->instersect(rayList[i+j*outImage.width],camera.getPosition()))
	// 				{
	// 					if(minDist > currentObject->lastIntersectionPoint.getDistanceTo(camera.getPosition()) )
	// 					{
	// 						minDist = currentObject->lastIntersectionPoint.getDistanceTo(camera.getPosition()) ;
	// 						colorForMinDist = currentObject->computeColorAtPoint(currentObject->lastIntersectionPoint);
	// 					}
	// 				}
	// 			}
	// 			if(minDist < DBL_MAX)
	// 			{
	// 				outImageData[(i+j*outImage.width)*4+0] = colorForMinDist.r;
	// 				outImageData[(i+j*outImage.width)*4+1] = colorForMinDist.g;
	// 				outImageData[(i+j*outImage.width)*4+2] = colorForMinDist.b;
	// 				outImageData[(i+j*outImage.width)*4+3] = colorForMinDist.a;
	// 			}
	// 			else
	// 			{
	// 				outImageData[(i+j*outImage.width)*4+0] = m_backgroundColor.r;
	// 				outImageData[(i+j*outImage.width)*4+1] = m_backgroundColor.g;
	// 				outImageData[(i+j*outImage.width)*4+2] = m_backgroundColor.b;
	// 				outImageData[(i+j*outImage.width)*4+3] = m_backgroundColor.a;
	// 			}

	// 		}
	// 	}
	// }

}