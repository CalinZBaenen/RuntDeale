macro_rules! impl_generic_traits {
    (Girthy for $x:ident) => {
        impl Girthy for $x {
            fn get_dimensions(&self) -> (Scalar, Scalar) {
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
            fn shift_position(&mut self, dx: Scalar, dy: Scalar) {
                self.x += dx;
                self.y += dy;
            }

            fn get_position(&self) -> (Scalar, Scalar) {
                (self.x, self.y)
            }

            fn set_position(&mut self, x: Scalar, y: Scalar) {
                self.x = x;
                self.y = y;
            }
        }
    };
    (EntityDescriptor for $x:ident) => {
        impl EntityDescriptor for $x {
            fn has_collision(&self) -> bool {
                self.is_ghost
            }
            fn set_collision(&mut self, f: bool) {
                self.is_ghost = f;
            }

            fn damage(&mut self, amt: i32) -> i32 {
                self.hp -= amt;
                self.hp
            }
            fn get_hp(&self) -> i32 {
                self.hp
            }
            fn set_hp(&mut self, amt: i32) {
                self.hp = amt;
            }
        }
    };
    (Actor for $x:ident) => {
        impl Actor for $x {
            fn set_velocity_x(&mut self, v: Scalar) {
                self.velocity_x = v;
            }

            fn set_velocity_y(&mut self, v: Scalar) {
                self.velocity_y = v;
            }

            fn set_velocity_by(&mut self, vx: Scalar, vy: Scalar) {
                self.velocity_x = vx;
                self.velocity_y = vy;
            }
        }
    };
}
