use graphics::math::Scalar;

/**
* Rules for how things are sized.
*/
pub trait Girthy {
    fn get_dimensions(&self) -> (Scalar, Scalar);

    fn resize_by(&mut self, dw: Scalar, dh: Scalar);
    fn resize(&mut self, w: Scalar, h: Scalar);
}

/**
* Rules for how things should be positioned.
*/
pub trait Positioned {
    fn shift_position(&mut self, dx: Scalar, dy: Scalar);

    fn get_position(&self) -> (Scalar, Scalar);
    fn set_position(&mut self, x: Scalar, y: Scalar);
}
