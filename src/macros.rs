macro_rules! impl_generic_traits {
    (Girthy for $x:ident) => {
        impl Girthy for $x {
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
        impl Positioned for $x {
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
    (EntityDescriptor for $x:ident) => {
        impl EntityDescriptor for $x {
            fn hasCollision(&self) -> bool {
                self.isGhost
            }
            fn setCollision(&mut self, f: bool) {
                self.isGhost = f;
            }

            fn damage(&mut self, amt: i32) -> i32 {
                self.hp -= amt;
                self.hp
            }
            fn getHP(&self) -> i32 {
                self.hp
            }
            fn setHP(&mut self, amt: i32) {
                self.hp = amt;
            }
        }
    };
    (Actor for $x:ident) => {
        impl Actor for $x {
            fn setVelocityX(&mut self, v: Scalar) {
                self.velocityX = v;
            }

            fn setVelocityY(&mut self, v: Scalar) {
                self.velocityY = v;
            }

            fn setVelocity_by(&mut self, vx: Scalar, vy: Scalar) {
                self.velocityX = vx;
                self.velocityY = vy;
            }
        }
    };
}
