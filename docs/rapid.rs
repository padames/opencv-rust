pub mod rapid {
	//! # silhouette based 3D object tracking
	//! 
	//! implements "RAPID-a video rate object tracker" [harris1990rapid](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_harris1990rapid) with the dynamic control point extraction of [drummond2002real](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_drummond2002real)
	use crate::{mod_prelude::*, core, sys, types};
	pub mod prelude {
		pub use { super::TrackerConst, super::Tracker, super::RapidConst, super::Rapid, super::OLSTrackerConst, super::OLSTracker, super::GOSTrackerConst, super::GOSTracker };
	}
	
	/// Collect corresponding 2d and 3d points based on correspondencies and mask
	/// ## Parameters
	/// * cols: correspondence-position per line in line-bundle-space
	/// * srcLocations: the source image location
	/// * pts2d: 2d points
	/// * pts3d: 3d points
	/// * mask: mask containing non-zero values for the elements to be retained
	/// 
	/// ## C++ default parameters
	/// * pts3d: noArray()
	/// * mask: noArray()
	#[inline]
	pub fn convert_correspondencies(cols: &dyn core::ToInputArray, src_locations: &dyn core::ToInputArray, pts2d: &mut dyn core::ToOutputArray, pts3d: &mut dyn core::ToInputOutputArray, mask: &dyn core::ToInputArray) -> Result<()> {
		extern_container_arg!(cols);
		extern_container_arg!(src_locations);
		extern_container_arg!(pts2d);
		extern_container_arg!(pts3d);
		extern_container_arg!(mask);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_convertCorrespondencies_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_const__InputArrayR(cols.as_raw__InputArray(), src_locations.as_raw__InputArray(), pts2d.as_raw__OutputArray(), pts3d.as_raw__InputOutputArray(), mask.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Debug draw markers of matched correspondences onto a lineBundle
	/// ## Parameters
	/// * bundle: the lineBundle
	/// * cols: column coordinates in the line bundle
	/// * colors: colors for the markers. Defaults to white.
	/// 
	/// ## C++ default parameters
	/// * colors: noArray()
	#[inline]
	pub fn draw_correspondencies(bundle: &mut dyn core::ToInputOutputArray, cols: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<()> {
		extern_container_arg!(bundle);
		extern_container_arg!(cols);
		extern_container_arg!(colors);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawCorrespondencies_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(bundle.as_raw__InputOutputArray(), cols.as_raw__InputArray(), colors.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Debug draw search lines onto an image
	/// ## Parameters
	/// * img: the output image
	/// * locations: the source locations of a line bundle
	/// * color: the line color
	#[inline]
	pub fn draw_search_lines(img: &mut dyn core::ToInputOutputArray, locations: &dyn core::ToInputArray, color: core::Scalar) -> Result<()> {
		extern_container_arg!(img);
		extern_container_arg!(locations);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawSearchLines_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(img.as_raw__InputOutputArray(), locations.as_raw__InputArray(), &color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draw a wireframe of a triangle mesh
	/// ## Parameters
	/// * img: the output image
	/// * pts2d: the 2d points obtained by [projectPoints]
	/// * tris: triangle face connectivity
	/// * color: line color
	/// * type: line type. See [LineTypes].
	/// * cullBackface: enable back-face culling based on CCW order
	/// 
	/// ## C++ default parameters
	/// * typ: LINE_8
	/// * cull_backface: false
	#[inline]
	pub fn draw_wireframe(img: &mut dyn core::ToInputOutputArray, pts2d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, color: core::Scalar, typ: i32, cull_backface: bool) -> Result<()> {
		extern_container_arg!(img);
		extern_container_arg!(pts2d);
		extern_container_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_drawWireframe_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR_const_ScalarR_int_bool(img.as_raw__InputOutputArray(), pts2d.as_raw__InputArray(), tris.as_raw__InputArray(), &color, typ, cull_backface, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extract control points from the projected silhouette of a mesh
	/// 
	/// see [drummond2002real](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_drummond2002real) Sec 2.1, Step b
	/// ## Parameters
	/// * num: number of control points
	/// * len: search radius (used to restrict the ROI)
	/// * pts3d: the 3D points of the mesh
	/// * rvec: rotation between mesh and camera
	/// * tvec: translation between mesh and camera
	/// * K: camera intrinsic
	/// * imsize: size of the video frame
	/// * tris: triangle face connectivity
	/// * ctl2d: the 2D locations of the control points
	/// * ctl3d: matching 3D points of the mesh
	#[inline]
	pub fn extract_control_points(num: i32, len: i32, pts3d: &dyn core::ToInputArray, rvec: &dyn core::ToInputArray, tvec: &dyn core::ToInputArray, k: &dyn core::ToInputArray, imsize: core::Size, tris: &dyn core::ToInputArray, ctl2d: &mut dyn core::ToOutputArray, ctl3d: &mut dyn core::ToOutputArray) -> Result<()> {
		extern_container_arg!(pts3d);
		extern_container_arg!(rvec);
		extern_container_arg!(tvec);
		extern_container_arg!(k);
		extern_container_arg!(tris);
		extern_container_arg!(ctl2d);
		extern_container_arg!(ctl3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_extractControlPoints_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const_SizeR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(num, len, pts3d.as_raw__InputArray(), rvec.as_raw__InputArray(), tvec.as_raw__InputArray(), k.as_raw__InputArray(), &imsize, tris.as_raw__InputArray(), ctl2d.as_raw__OutputArray(), ctl3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extract the line bundle from an image
	/// ## Parameters
	/// * len: the search radius. The bundle will have `2*len + 1` columns.
	/// * ctl2d: the search lines will be centered at this points and orthogonal to the contour defined by
	/// them. The bundle will have as many rows.
	/// * img: the image to read the pixel intensities values from
	/// * bundle: line bundle image with size `ctl2d.rows() x (2 * len + 1)` and the same type as @p img
	/// * srcLocations: the source pixel locations of @p bundle in @p img as CV_16SC2
	#[inline]
	pub fn extract_line_bundle(len: i32, ctl2d: &dyn core::ToInputArray, img: &dyn core::ToInputArray, bundle: &mut dyn core::ToOutputArray, src_locations: &mut dyn core::ToOutputArray) -> Result<()> {
		extern_container_arg!(ctl2d);
		extern_container_arg!(img);
		extern_container_arg!(bundle);
		extern_container_arg!(src_locations);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_extractLineBundle_int_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(len, ctl2d.as_raw__InputArray(), img.as_raw__InputArray(), bundle.as_raw__OutputArray(), src_locations.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Find corresponding image locations by searching for a maximal sobel edge along the search line (a single
	/// row in the bundle)
	/// ## Parameters
	/// * bundle: the line bundle
	/// * cols: correspondence-position per line in line-bundle-space
	/// * response: the sobel response for the selected point
	/// 
	/// ## C++ default parameters
	/// * response: noArray()
	#[inline]
	pub fn find_correspondencies(bundle: &dyn core::ToInputArray, cols: &mut dyn core::ToOutputArray, response: &mut dyn core::ToOutputArray) -> Result<()> {
		extern_container_arg!(bundle);
		extern_container_arg!(cols);
		extern_container_arg!(response);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_findCorrespondencies_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(bundle.as_raw__InputArray(), cols.as_raw__OutputArray(), response.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// High level function to execute a single rapid [harris1990rapid](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_harris1990rapid) iteration
	/// 
	/// 1. [extractControlPoints]
	/// 2. [extractLineBundle]
	/// 3. [findCorrespondencies]
	/// 4. [convertCorrespondencies]
	/// 5. [solvePnPRefineLM]
	/// 
	/// ## Parameters
	/// * img: the video frame
	/// * num: number of search lines
	/// * len: search line radius
	/// * pts3d: the 3D points of the mesh
	/// * tris: triangle face connectivity
	/// * K: camera matrix
	/// * rvec: rotation between mesh and camera. Input values are used as an initial solution.
	/// * tvec: translation between mesh and camera. Input values are used as an initial solution.
	/// * rmsd: the 2d reprojection difference
	/// ## Returns
	/// ratio of search lines that could be extracted and matched
	/// 
	/// ## C++ default parameters
	/// * rmsd: 0
	#[inline]
	pub fn rapid(img: &dyn core::ToInputArray, num: i32, len: i32, pts3d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, k: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, rmsd: &mut f64) -> Result<f32> {
		extern_container_arg!(img);
		extern_container_arg!(pts3d);
		extern_container_arg!(tris);
		extern_container_arg!(k);
		extern_container_arg!(rvec);
		extern_container_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_rapid_const__InputArrayR_int_int_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_doubleX(img.as_raw__InputArray(), num, len, pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), rmsd, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Constant methods for [crate::rapid::GOSTracker]
	pub trait GOSTrackerConst: crate::rapid::TrackerConst {
		fn as_raw_GOSTracker(&self) -> *const c_void;
	
	}
	
	/// implements "Global optimal searching for textureless 3D object tracking" [wang2015global](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_wang2015global)
	pub trait GOSTracker: crate::rapid::GOSTrackerConst + crate::rapid::Tracker {
		fn as_raw_mut_GOSTracker(&mut self) -> *mut c_void;
	
	}
	
	impl dyn GOSTracker + '_ {
		/// ## C++ default parameters
		/// * hist_bins: 4
		/// * sobel_thesh: 10
		#[inline]
		pub fn create(pts3d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<dyn crate::rapid::OLSTracker>> {
			extern_container_arg!(pts3d);
			extern_container_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<dyn crate::rapid::OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	/// Constant methods for [crate::rapid::OLSTracker]
	pub trait OLSTrackerConst: crate::rapid::TrackerConst {
		fn as_raw_OLSTracker(&self) -> *const c_void;
	
	}
	
	/// implements "Optimal local searching for fast and robust textureless 3D object tracking in highly
	/// cluttered backgrounds" [seo2013optimal](https://docs.opencv.org/4.7.0/d0/de3/citelist.html#CITEREF_seo2013optimal)
	pub trait OLSTracker: crate::rapid::OLSTrackerConst + crate::rapid::Tracker {
		fn as_raw_mut_OLSTracker(&mut self) -> *mut c_void;
	
	}
	
	impl dyn OLSTracker + '_ {
		/// ## C++ default parameters
		/// * hist_bins: 8
		/// * sobel_thesh: 10
		#[inline]
		pub fn create(pts3d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<dyn crate::rapid::OLSTracker>> {
			extern_container_arg!(pts3d);
			extern_container_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<dyn crate::rapid::OLSTracker>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	/// Constant methods for [crate::rapid::Rapid]
	pub trait RapidConst: crate::rapid::TrackerConst {
		fn as_raw_Rapid(&self) -> *const c_void;
	
	}
	
	/// wrapper around [rapid] function for uniform access
	pub trait Rapid: crate::rapid::RapidConst + crate::rapid::Tracker {
		fn as_raw_mut_Rapid(&mut self) -> *mut c_void;
	
	}
	
	impl dyn Rapid + '_ {
		#[inline]
		pub fn create(pts3d: &dyn core::ToInputArray, tris: &dyn core::ToInputArray) -> Result<core::Ptr<dyn crate::rapid::Rapid>> {
			extern_container_arg!(pts3d);
			extern_container_arg!(tris);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			let ret = unsafe { core::Ptr::<dyn crate::rapid::Rapid>::opencv_from_extern(ret) };
			Ok(ret)
		}
		
	}
	/// Constant methods for [crate::rapid::Tracker]
	pub trait TrackerConst: core::AlgorithmTraitConst {
		fn as_raw_Tracker(&self) -> *const c_void;
	
	}
	
	/// Abstract base class for stateful silhouette trackers
	pub trait Tracker: core::AlgorithmTrait + crate::rapid::TrackerConst {
		fn as_raw_mut_Tracker(&mut self) -> *mut c_void;
	
		/// ## C++ default parameters
		/// * termcrit: TermCriteria(TermCriteria::MAX_ITER|TermCriteria::EPS,5,1.5)
		#[inline]
		fn compute(&mut self, img: &dyn core::ToInputArray, num: i32, len: i32, k: &dyn core::ToInputArray, rvec: &mut dyn core::ToInputOutputArray, tvec: &mut dyn core::ToInputOutputArray, termcrit: core::TermCriteria) -> Result<f32> {
			extern_container_arg!(img);
			extern_container_arg!(k);
			extern_container_arg!(rvec);
			extern_container_arg!(tvec);
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(self.as_raw_mut_Tracker(), img.as_raw__InputArray(), num, len, k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &termcrit, ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
		#[inline]
		fn clear_state(&mut self) -> Result<()> {
			return_send!(via ocvrs_return);
			unsafe { sys::cv_rapid_Tracker_clearState(self.as_raw_mut_Tracker(), ocvrs_return.as_mut_ptr()) };
			return_receive!(unsafe ocvrs_return => ret);
			let ret = ret.into_result()?;
			Ok(ret)
		}
		
	}
}
