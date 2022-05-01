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
    (Positioned for $x:ident) => {
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
