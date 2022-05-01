use graphics::math::Scalar;

/**
* Rules for how things are sized.
*/
pub trait Girthy {
    fn getDimensions(&self) -> (Scalar, Scalar);

    fn resize(&mut self, dw: Scalar, dh: Scalar) -> (Scalar, Scalar);

    fn setH(&mut self, h: Scalar);
    fn setW(&mut self, w: Scalar);
}

/**
* Rules for how things should be positioned.
*/
pub trait Positioned {
    fn shiftPosition(&self, dx: Scalar, dy: Scalar) -> (Scalar, Scalar);

    fn getPosition(&self) -> (Scalar, Scalar);
    fn setPosition(&mut self, x: Scalar, y: Scalar);

    fn setX(&mut self, x: Scalar);
    fn setY(&mut self, y: Scalar);
}
