use graphics::math::Scalar;

/**
* Rules for how things are sized.
*/
pub trait Girthy {
    fn getDimensions(&self) -> (Scalar, Scalar);

    fn resize_by(&mut self, dw: Scalar, dh: Scalar);
    fn resize(&mut self, w: Scalar, h: Scalar);
}

/**
* Rules for how things should be positioned.
*/
pub trait Positioned {
    fn shiftPosition(&mut self, dx: Scalar, dy: Scalar);

    fn getPosition(&self) -> (Scalar, Scalar);
    fn setPosition(&mut self, x: Scalar, y: Scalar);
}
