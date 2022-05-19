use simba;
mod vector3 {
    use std::ops::Add;
    use std::ops::Sub;

    #[derive(Debug, PartialEq)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    
    impl Vector3{
        pub fn magnitude (&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }

        pub fn cross_product(&self, rhs: &Vector3) -> Vector3 {
            let x = self.y*rhs.z - self.z*rhs.y;
            let y = self.z*rhs.x - self.x*rhs.z;
            let z = self.x*rhs.y - self.y*rhs.x;
            let cross = Vector3 {x, y, z};
            return cross;
        }

        pub fn dot_product(&self, rhs: &Vector3) -> f64 {
            return self.x*rhs.x + self.y*rhs.y + self.z*rhs.z;
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

    impl Eq for Vector3 {}
}

mod particle {
    use crate::vector3::Vector3;

    pub struct Particle {
        position: Vector3,
        velocity: Vector3,
        acceleration: Vector3,

        damping: f64,
        inverse_mass: f64
    }


}


fn main() {
    let cat: simba::scalar = 42;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::Vector3;

    #[test]
    fn verify_cross_product() {
        let x: vector3::Vector3 = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let y: vector3::Vector3 = Vector3 {x: 0.0, y: 1.0, z: 0.0};
        let z: vector3::Vector3 = Vector3 {x: 0.0, y: 0.0, z: 1.0};
        assert_eq!(x.cross_product(&y), z);
        assert_eq!(y.cross_product(&z), x);
        assert_eq!(z.cross_product(&x), y);
    }

    #[test]
    fn verify_dot_product() {
        let x: vector3::Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0};
        let y: vector3::Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0};
        let z: vector3::Vector3 = Vector3{x: 0.0, y: 0.0, z: 1.0};
        assert_eq!(x.dot_product(&y), 0.0);
        assert_eq!(y.dot_product(&z), 0.0);
        assert_eq!(x.dot_product(&z), 0.0);
    }

}

