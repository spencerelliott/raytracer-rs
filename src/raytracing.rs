use std::fmt;
use std::ops;

pub fn check_for_hit<'a>(
    hitables: &Vec<&dyn Hitable<'a>>,
    ray: &Ray,
    time_min: f32,
    time_max: f32,
) -> Option<HitRecord<'a>> {
    let mut hit_record: Option<HitRecord> = None;
    let mut closest = time_max;

    for hitable in hitables {
        let hit_result = hitable.hit(ray, time_min, closest);

        match hit_result {
            Some(hit_result) => {
                closest = hit_result.time;
                hit_record = Some(hit_result);
            }
            None => {}
        }
    }

    hit_record
}

/// Describes a collision between a `Ray` and a point
#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub time: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
}

pub trait Material {
    /// Calculates the next bounced ray after a hit at a point and returns
    /// whether or not the ray has been absorbed
    ///
    /// # Arguments
    ///
    /// * `ray` - The initial ray direction
    /// * `hit_record` - The latest information about where the `Ray` hit last
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3)>;
}

pub trait Hitable<'a> {
    /// Determines and returns whether a ray intersects this object
    ///
    /// # Arguments
    ///
    /// * `ray` - The incoming ray direction
    /// * `time_min` - The minimum time value in which the ray will register a hit
    /// * `time_max` - The maximum time value in which the ray will register a hit
    /// * `hit_record` - The hit record that will contain all information about the hit
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitRecord<'a>>;
}

/// Represents a vector with an initial point and a direction
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl fmt::Display for Ray {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Ray[{}, {}]", self.origin, self.direction))
    }
}

impl Ray {
    /// Calculates a point along the long based on time
    ///
    /// # Arguments
    ///
    /// * `time` - a floating point number representing the position along the line
    ///
    /// # Returns
    /// A new `Vector3` representing the point along the line
    pub fn point_at_parameter(self, time: f32) -> Vector3 {
        self.origin + (self.direction * time)
    }
}

/// Represents a point in space
#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Vector3 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Vec3[{}, {}, {}]", self.x, self.y, self.z))
    }
}

impl Vector3 {
    pub fn squared_length(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn dot(self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn modify<F>(self, modification: F) -> Vector3
    where
        F: Fn(f32) -> f32,
    {
        Vector3 {
            x: modification(self.x),
            y: modification(self.y),
            z: modification(self.z),
        }
    }

    /// Returns a normalized `Vector3`
    pub fn as_unit_vector(self) -> Vector3 {
        let k = 1.0 / self.length();

        Vector3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub const ONE: Vector3 = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const ZERO: Vector3 = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Vector3 {
        _rhs * self
    }
}

impl ops::Div for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, _rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}
