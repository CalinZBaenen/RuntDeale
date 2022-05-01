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

macro_rules! impl_generic_traits {
    (Girthy for $x:ident) => {
        impl Girthy for Entity {
            fn getDimensions(&self) -> (Scalar, Scalar) {
                (self.w, self.h)
            }

            fn resize_by(&mut self, dw: Scalar, dh: Scalar) {
                self.h += dh;
                self.w += dw;
            }

            fn resize(&mut self, w: Scalar, h: Scalar) {
                self.h = h;
                self.w = w;
            }
        }
    };
    (Position for $x:ident) => {
        impl Positioned for Entity {
            fn shiftPosition(&mut self, dx: Scalar, dy: Scalar) {
                self.x += dx;
                self.y += dy;
            }

            fn getPosition(&self) -> (Scalar, Scalar) {
                (self.x, self.y)
            }

            fn setPosition(&mut self, x: Scalar, y: Scalar) {
                self.x = x;
                self.y = y;
            }
        }
    };
}
