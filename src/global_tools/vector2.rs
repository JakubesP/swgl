use nalgebra_glm as glm;

use std::convert::From;
use std::convert::Into;
use std::default::Default;
use std::ops;

// -----------------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// -----------------------------------------------------------------------------------------------------------

impl<T> Vector2<T>
where
    T: num_traits::float::Float + num_traits::cast::FromPrimitive,
{
    /// This method returns distance between 2D points.
    pub fn distance_to(&self, other: &Self) -> T {
        let diff_x = self.x - other.x;
        let diff_y = self.y - other.y;
        (diff_x * diff_x + diff_y * diff_y).sqrt()
    }

    /// This method returns vector magnitude (length).
    pub fn mag(&mut self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// This method specifies vector magnitude (length).
    pub fn set_mag(&mut self, new_mag: T) {
        let current_mag = self.mag();
        let zero = num_traits::cast::FromPrimitive::from_f32(0.0).unwrap();
        if current_mag != zero && new_mag != zero {
            self.x = self.x * new_mag / current_mag;
            self.y = self.y * new_mag / current_mag;
        }
    }

    /// This method limits vector magnitude to specified value. 
    pub fn limit(&mut self, max_mag: T) {
        if self.mag() > max_mag {
           self.set_mag(max_mag);
        } 
    }

    /// This method returns normalized copy of vector.
    pub fn normalized(&mut self) -> Self {
        let length = self.mag();
        Self::new(self.x / length, self.y / length)
    }

    /// This method normalizes vector in place.
    pub fn normalize(&mut self) {
        let normalized = self.normalized();
        self.x = normalized.x;
        self.y = normalized.y;
    }

    /// This method returns copy of vector rotated around specific point.  
    pub fn rotated_around(&self, angle: T, origin: &Self) -> Self {
        let temp = *origin - *self;
        let x = temp.x * angle.cos() - temp.y * angle.sin();
        let y = temp.x * angle.sin() + temp.y * angle.cos();
        Vector2::new(x, y) + *origin
    }

    /// This metod rotates vector around specific point in place.
    pub fn rotate_around(&mut self, angle: T, origin: &Self) {
        let rotated = self.rotated_around(angle, origin);
        self.x = rotated.x;
        self.y = rotated.y;
    }

    /// This method returns scalar product of two vectors.
    pub fn scalar_product(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// This method returns vector as direction (angle) in radians.
    pub fn heading(&self) -> T {
        self.y.atan2(self.x)
    }

    /// This methods creates new vector from radians direction (angle).
    pub fn from_angle(angle: T) -> Self {
        Self {
            x: angle.cos(),
            y: angle.sin(),
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// constructor

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector2<T>
where
    T: Default,
{
    pub fn zero() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// add

impl<T> ops::Add for Vector2<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> ops::AddAssign for Vector2<T>
where
    T: ops::Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// sub

impl<T> ops::Sub for Vector2<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> ops::SubAssign for Vector2<T>
where
    T: ops::Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// mult

impl<T> ops::Mul for Vector2<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> ops::MulAssign for Vector2<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// mult by scalar

impl<T> ops::Mul<T> for Vector2<T>
where
    T: ops::Mul<Output = T>,
    T: Copy + Clone,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> ops::MulAssign<T> for Vector2<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// divide by scalar

impl<T> ops::Div<T> for Vector2<T>
where
    T: ops::Div<Output = T>,
    T: Copy + Clone,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> ops::DivAssign<T> for Vector2<T>
where
    T: ops::Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// neg

impl<T> ops::Neg for Vector2<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// glm::Vec2 intergation

impl<T> From<glm::TVec2<T>> for Vector2<T>
where
    T: Copy,
    T: nalgebra_glm::Scalar,
{
    fn from(vec: glm::TVec2<T>) -> Self {
        Vector2::new(vec.x, vec.y)
    }
}

impl<T> Into<glm::TVec2<T>> for Vector2<T>
where
    T: Copy,
    T: nalgebra_glm::Scalar,
{
    fn into(self) -> glm::TVec2<T> {
        glm::TVec2::new(self.x, self.y)
    }
}

// -----------------------------------------------------------------------------------------------------------
// glm::Vec3 intergation

impl<T> From<glm::TVec3<T>> for Vector2<T>
where
    T: Copy,
    T: nalgebra_glm::Scalar,
{
    fn from(vec: glm::TVec3<T>) -> Self {
        Vector2::new(vec.x, vec.y)
    }
}

impl<T> Into<glm::TVec3<T>> for Vector2<T>
where
    T: Copy,
    T: nalgebra_glm::Scalar,
    T: Default,
{
    fn into(self) -> glm::TVec3<T> {
        glm::TVec3::new(self.x, self.y, T::default())
    }
}
