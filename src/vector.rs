// TODO: left-commutative operations with scalars
// TODO: better c-tor
// TODO: add new() method
extern crate rand;
use rand::distributions::{Distribution, Normal};

use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vector3f {
    type Output = Vector3f;

    // TODO: why not reference here?
    fn add(self, other: Vector3f) -> Vector3f {
        Vector3f{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector3f {
    type Output = Vector3f;

    fn sub(self, other: Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vector3f {
    type Output = Vector3f;

    fn neg(self) -> Vector3f {
        Vector3f {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f32> for Vector3f {
    type Output = Vector3f;

    fn mul(self, w: f32) -> Vector3f {
        Vector3f {
            x: self.x * w,
            y: self.y * w,
            z: self.z * w
        }
    }
}

impl Div<f32> for Vector3f {
    type Output = Vector3f;

    fn div(self, w: f32) -> Vector3f {
        Vector3f {
            x: self.x / w,
            y: self.y / w,
            z: self.z / w
        }
    }
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {x: x, y: y, z: z}
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn norm(&self) -> f32 {
        self.length()
    }

    pub fn dot(&self, other: Vector3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[allow(dead_code)]
    pub fn cross(&self, other: Vector3f) -> Vector3f {
        Vector3f{
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn normalized(&self) -> Vector3f {
        let n = self.norm();
        if n > 0.0 {
            return *self / n
        }
        else {
            return self.clone();
        }
    }

    pub fn random_unit() -> Vector3f {
        let normal = Normal::new(0.0, 1.0);
        let x = normal.sample(&mut rand::thread_rng()) as f32;
        let y = normal.sample(&mut rand::thread_rng()) as f32;
        let z = normal.sample(&mut rand::thread_rng()) as f32;
        Vector3f::new(x, y, z).normalized()
    }
}

// tests go below
#[test]
fn test_add(){
    let v0 = Vector3f{
        x: 0.0,
        y: -1.0,
        z: 5.5
    };
    let v1 = Vector3f{
        x: 0.0,
        y: 1.0,
        z: -5.5
    };
    let v2 = Vector3f{
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    assert_eq!(v0 + v1, v2)
}

#[test]
fn test_sub(){
    let v0 = Vector3f{
        x: 0.0,
        y: -1.0,
        z: 5.5
    };
    let v1 = Vector3f{
        x: 0.0,
        y: 1.0,
        z: -5.5
    };
    let v2 = Vector3f{
        x: 0.0,
        y: -2.0,
        z: 11.0
    };
    assert_eq!(v0 - v1, v2)
}

#[test]
fn test_norm_triange(){
    let v0 = Vector3f{
        x: 0.0, y: 1.0, z: 0.0
    };
    let v1 = Vector3f{
        x: 0.0, y: 0.0, z: -1.0
    };
    assert!((v0 + v1).norm() <= v0.norm() + v1.norm())
}

#[test]
fn test_dot(){
    let a = Vector3f{x: 1.0, y: 0.0, z: 0.0};
    let b = Vector3f{x: 7.0, y: 10.0, z: -4.2};

    assert_eq!(a.dot(b), 7.0)
}

#[test]
fn test_normalized(){
    let a = Vector3f{x: -3.0, y: 12.4, z: 0.0003};
    assert!((1.0 - a.normalized().norm()).abs() <= 1e-5)
}
