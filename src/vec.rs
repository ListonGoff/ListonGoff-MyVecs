#![allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const UP: Vec2 = Vec2 { x: 0., y: 1. };

    pub const DOWN: Vec2 = Vec2 { x: 0., y: -1. };

    pub const RIGHT: Vec2 = Vec2 { x: 1., y: 0. };

    pub const LEFT: Vec2 = Vec2 { x: -1., y: 0. };

    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn splat(x: f32) -> Vec2 {
        Vec2 { x, y: x }
    }

    pub fn max(vec1: Vec2, vec2: Vec2) -> Vec2 {
        let x: f32 = if vec1.x > vec2.x { vec1.x } else { vec2.x };
        let y: f32 = if vec1.y > vec2.y { vec1.y } else { vec2.y };

        Vec2 { x, y }
    }

    pub fn min(vec1: Vec2, vec2: Vec2) -> Vec2 {
        let x: f32 = if vec1.x < vec2.x { vec1.x } else { vec2.x };
        let y: f32 = if vec1.y < vec2.y { vec1.y } else { vec2.y };

        Vec2 { x, y }
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.,
        }
    }

    pub fn magnitude(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }

    pub fn direction(&self) -> f32 {
        let radian = (self.y / self.x).atan();

        radian * 180. / 3.14
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, rhs: &Self) -> bool {
        return (self.x == rhs.x) && (self.y == rhs.y);
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: Self) -> Self::Output {
        Vec2 {
            x: _rhs.x * self.x,
            y: _rhs.y * self.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Self) -> Self::Output {
        Vec2 {
            x: _rhs.x + self.x,
            y: _rhs.y + self.y,
        }
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}

impl std::ops::Div for Vec2 {
    type Output = Vec2;
    
    fn div(self, rhs: Self) -> Self::Output {
       Vec2 {
           x: self.x / rhs.x,
           y: self.y / rhs.y,
       }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub const UP: Vec3 = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub const DOWN: Vec3 = Vec3 {
        x: 0.,
        y: -1.,
        z: 0.,
    };

    pub const RIGHT: Vec3 = Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    };

    pub const LEFT: Vec3 = Vec3 {
        x: -1.,
        y: 0.,
        z: 0.,
    };

    pub const FORWARD: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub const BACKWARD: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: -1.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn splat(x: f32) -> Vec3 {
        Vec3 { x, y: x, z: x }
    }

    pub fn max(vec1: Vec3, vec2: Vec3) -> Vec3 {
        let x: f32 = if vec1.x > vec2.x { vec1.x } else { vec2.x };
        let y: f32 = if vec1.y > vec2.y { vec1.y } else { vec2.y };
        let z: f32 = if vec1.z > vec2.z { vec1.z } else { vec2.z };

        Vec3 { x, y, z }
    }

    pub fn min(vec1: Vec3, vec2: Vec3) -> Vec3 {
        let x: f32 = if vec1.x < vec2.x { vec1.x } else { vec2.x };
        let y: f32 = if vec1.y < vec2.y { vec1.y } else { vec2.y };
        let z: f32 = if vec1.z < vec2.z { vec1.z } else { vec2.z };

        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        let t = (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
        t
    }
    pub fn sum(&self) -> f32 {
        return self.x + self.y + self.z;
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn magnitude(&self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        return (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z);
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Add<Vec2> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec3 {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: self.z,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}