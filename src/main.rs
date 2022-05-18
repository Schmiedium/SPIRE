
mod vector3 {
    use std::ops::Add;
    use std::ops::Sub;
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vector3{
        pub fn magnitude (&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }

        pub fn cross_product(&self, vec2: &Vector3) -> Vector3 {
            let x = self.y*vec2.z - self.z*vec2.y;
            let y = self.z*vec2.x - self.x*vec2.z;
            let z = self.x*vec2.y - self.y*vec2.x;
            let cross = Vector3 {x, y, z};
            return cross;
        }

        pub fn dot_product(&self, vec2: &Vector3) -> f64 {
            return self.x*vec2.x + self.y*vec2.y + self.z*vec2.z;
        }

        pub fn normalize(&self) -> Self{
            let mag = self.magnitude();
            let x = self.x/mag;
            let y = self.y/mag;
            let z = self.z/mag;
            return Vector3{x, y, z}
        }
    }
    
    impl Add for Vector3 {
        type Output =  Vector3;

        fn add(self, rhs: Vector3) -> Vector3 {
            Vector3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
        }
    }

    impl Sub for Vector3 {
        type Output = Vector3;

        fn sub(self, rhs: Vector3) -> Vector3 {
            Vector3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}            
        }
    }
}

mod particle {
    use crate::vector3::Vector3;

    pub struct Particle {
        position: Vector3,
        velocity: Vector3,
        acceleration: Vector3,
    }
}


fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cross_product() {
        assert_eq!();
    }

    #[test]
    fn verify_dot_product() {
        assert_eq!();
    }

}

