use std::option::Option;

use graphics::{math::Scalar, Graphics};

use super::position::{Girthy, Positioned};

/**
  Defines methods for interacting with collision.
*/
pub trait Collision: Positioned + Girthy {
    fn overlapping(&mut self, other: &mut Self) -> bool {
        let opos = other.getPosition();
        let spos = self.getPosition();
        let osz = self.getDimensions();
        let ssz = self.getDimensions();
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
    fn hasCollision(&self) -> bool;
    fn setCollision(&mut self, f: bool);

    fn damage(&mut self, amt: i32) -> i32;
    fn getHP(&self) -> i32;
    fn setHP(&mut self, amt: i32);
}

/**
  Basic RuntDeale actor object.
  Provides functionality for moving
  around, getting X and Y coordinates,
  changing velocity, and (probably) more.
*/
pub trait Actor: Positioned {
    fn setVelocityX(&mut self, v: Scalar);
    fn setVelocityY(&mut self, v: Scalar);
    fn setVelocity(&mut self, vx: Scalar, vy: Scalar);
}

/**
  Represents an entity. An actor who has health,
  inventory, or other state.
*/
pub struct Entity {
    pub velocityX: Scalar,
    pub velocityY: Scalar,
    pub isGhost: bool,
    pub maxHp: i32,
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
            velocityX: 0.,
            velocityY: 0.,
            isGhost: true,
            maxHp: 0,
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
            velocityX: 0.,
            velocityY: 0.,
            isGhost: true,
            maxHp: 0,
            name: Option::None,
            hp: 0,
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
        }
    }
}

/**
  Implement the functions needed to describe an entity.
*/
impl EntityDescriptor for Entity {
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

impl Positioned for Entity {
    //
}

impl Collision for Entity {
    //
}

impl Girthy for Entity {
    fn getDimensions(&self) -> (Scalar, Scalar) {
        //
    }

    fn resize(&mut self, dx: Scalar, dy: Scalar) -> (Scalar, Scalar) {
        //
    }

    fn setH(&mut self, h: Scalar) {
        //
    }
    fn setW(&mut self, w: Scalar) {
        //
    }
}

impl Actor for Entity {
    fn setVelocityX(&mut self, v: Scalar) {
        self.velocityX = v;
    }

    fn setVelocityY(&mut self, v: Scalar) {
        self.velocityY = v;
    }

    fn setVelocity(&mut self, vx: Scalar, vy: Scalar) {
        self.velocityX = vx;
        self.velocityX = vy;
    }
}
