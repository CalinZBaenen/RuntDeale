use std::option::Option;

use graphics::math::Scalar;

use super::position::{Girthy, Positioned};

/**
  Defines methods for interacting with collision.
*/
pub trait Collision: Positioned + Girthy {
    fn overlapping(&mut self, other: &mut Self) -> bool {
        let opos = other.get_position();
        let spos = self.get_position();
        let osz = self.get_dimensions();
        let ssz = self.get_dimensions();
        true
        //if {}
        //if {}
        //if {}
        //if {}
    }
}

/**
  Describes an entity.
*/
pub trait EntityDescriptor: Collision {
    fn has_collision(&self) -> bool;
    fn set_collision(&mut self, f: bool);

    fn damage(&mut self, amt: i32) -> i32;
    fn get_hp(&self) -> i32;
    fn set_hp(&mut self, amt: i32);
}

/**
  Basic RuntDeale actor object.
  Provides functionality for moving
  around, getting X and Y coordinates,
  changing velocity, and (probably) more.
*/
pub trait Actor: Positioned {
    fn set_velocity_x(&mut self, v: Scalar);
    fn set_velocity_y(&mut self, v: Scalar);
    fn set_velocity_by(&mut self, vx: Scalar, vy: Scalar);
}

/**
  Represents an entity. An actor who has health,
  inventory, or other state.
*/
pub struct Entity {
    pub velocity_x: Scalar,
    pub velocity_y: Scalar,
    pub is_ghost: bool,
    pub max_hp: i32,
    pub name: Option<String>,
    pub hp: i32,
    pub w: Scalar,
    pub h: Scalar,
    x: Scalar,
    y: Scalar,
}

impl Entity {
    pub fn create(name: String, x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> Self {
        Self {
            velocity_x: 0.,
            velocity_y: 0.,
            is_ghost: true,
            max_hp: 0,
            name: Option::Some(name),
            hp: 0,
            x,
            y,
            w,
            h,
        }
    }

    pub fn new() -> Self {
        Self {
            velocity_x: 0.,
            velocity_y: 0.,
            is_ghost: true,
            max_hp: 0,
            name: Option::None,
            hp: 0,
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
        }
    }
}

impl_generic_traits!(Positioned for Entity);
impl_generic_traits!(Girthy for Entity);
impl_generic_traits!(EntityDescriptor for Entity);
impl_generic_traits!(Actor for Entity);

impl Collision for Entity {}
