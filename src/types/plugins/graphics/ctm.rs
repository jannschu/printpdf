//! Current transformation matrix, for transforming shapes (rotate, translate, scale)

use lopdf;
use traits::IntoPdfStreamOperation;

/// PDF transformation matrix. Once set, will operate on all following shapes,
/// until the `layer.restore_graphics_state()` is called. It is important to 
/// call `layer.save_graphics_state()` earlier.
#[derive(Debug)]
pub struct CurrentTransformationMatrix {
	/// Translate the shape in x (mm)
	pub translate_x: f64,
	/// Translate the shape in y (mm)
	pub translate_y: f64,
	/// Scale the shape in x (mm)
	pub scale_x: f64,
	/// Scale the shape in y (mm)
	pub scale_y: f64,
	/// Rotate the shape counter-clockwise by an angle (degree)
	pub rotation_ccw_angle: f64,
}

impl CurrentTransformationMatrix {
	/// Creates a new transformation matrix
	pub fn new(translate_x: f64, translate_y: f64, scale_x: f64, scale_y: f64, rotation_ccw_angle: f64)
	-> Self
	{
		Self {
			translate_x,
			translate_y,
			scale_x,
			scale_y,
			rotation_ccw_angle,
		}
	}

	/// Returns a default CTM that does nothing.
	pub fn default()
	-> Self
	{
		Self {
			translate_x: 0.0,
			translate_y: 0.0,
			scale_x: 1.0,
			scale_y: 1.0,
			rotation_ccw_angle: 0.0,
		}
	}
}

impl IntoPdfStreamOperation for CurrentTransformationMatrix {
	
	/// Consumes the object and converts it to an PDF stream operation
	fn into_stream_op(self: Box<Self>)
	-> Vec<lopdf::content::Operation>
	{
		use lopdf::Object::*;
		let rotation_rad = self.rotation_ccw_angle.to_radians();

		let cos_x = rotation_rad.cos();
		let sin_x = rotation_rad.sin();

		let cur_translate_x = mm_to_pt!(self.translate_x);
		let cur_translate_y = mm_to_pt!(self.translate_y);
		let cur_scale_x = mm_to_pt!(self.scale_x);
		let cur_scale_y = mm_to_pt!(self.scale_y);

		vec![lopdf::content::Operation::new("cm", vec![
			Real(cur_scale_x + cos_x), 
			Real(sin_x), 
			Real(-sin_x), 
			Real(cur_scale_y + cos_x), 
			Real(cur_translate_x), 
			Real(cur_translate_y)])]
	}
}