mod boxfilter;
mod gaussian;
mod mitchell;
mod triangle;

pub use self::boxfilter::BoxFilter;
pub use self::gaussian::GaussianFilter;
pub use self::mitchell::MitchellNetravali;
pub use self::triangle::TriangleFilter;

pub trait Filter: Send + Sync {
    fn evaluate(&self, x: f32, y: f32) -> f32;
    /// Return the x_width and y_width of the filter. The width is the distance from the origin to
    /// the cutoff point. The support or  extent of the filter (in one direction), is the total
    /// domain where the filter is non-zero. Extent = 2*width.
    fn width(&self) -> (f32, f32);
    fn inv_width(&self) -> (f32, f32);
}
