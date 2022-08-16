mod vector3 {
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::MulAssign;
    use std::ops::Rem;
    use std::ops::Sub;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    
    impl Vector3{
        pub fn magnitude (&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }


        pub fn normalize(&self) -> Self{
            let mag = self.magnitude();
            let x = self.x/mag;
            let y = self.y/mag;
            let z = self.z/mag;
            return Vector3{x, y, z}
        }

        pub fn invert(&self) -> Self{
            let x =  -self.x;
            let y =  -self.x;
            let z =  -self.x;
            return Vector3 { x: x, y: y, z: z }

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

    ///overloads * operator for scalar multiplication
    impl Mul<f64> for Vector3 {
        type Output = Vector3;

        fn mul(self, rhs: f64) -> Vector3 {
            return Vector3 { x: self.x*rhs, y: self.y*rhs, z: self.z*rhs }
        }
    }

    ///overloads *= operator for scalar multiplication and assignment
    impl MulAssign<f64> for Vector3 {

        fn mul_assign(&mut self, rhs: f64) {
            self.x = self.x*rhs;
            self.y = self.y*rhs;
            self.z = self.z*rhs;
        }
    }

    ///overloads * operator for dot product multiplication
    impl Mul for Vector3 {
        type Output = f64;

        fn mul(self, rhs: Self) -> f64 {
            return (self.x*rhs.x + self.y*rhs.y + self.z*rhs.z);
        }
    }

    ///overloads the % operator for cross product multiplicationS
    impl Rem for Vector3 {
        type Output = Vector3;

        fn rem(self, rhs: Self) -> Vector3 {
            let x = self.y*rhs.z - self.z*rhs.y;
            let y = self.z*rhs.x - self.x*rhs.z;
            let z = self.x*rhs.y - self.y*rhs.x;
            let cross = Vector3 {x, y, z};
            return cross;
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

    impl Particle {

        /// Runge Kutta 4 implementation for a 
        /// 3 degrees of freedommodel 
        fn update_position(&mut self, wind_vector: &Vector3, grav: &f64, drag_coefficient: &f64, dt: &f64) -> () {
            let v11: f64 = (self.velocity - *wind_vector).magnitude();
            
            let accel1: Vector3 = Vector3 { 
                x: (self.velocity - *wind_vector).x*v11*drag_coefficient, 
                y: (self.velocity - *wind_vector).y*v11*drag_coefficient + grav, 
                z: (self.velocity - *wind_vector).z*v11*drag_coefficient 
            };

            let velocity2: Vector3 = self.velocity - accel1**dt;
            let v12: f64 = (velocity2 - *wind_vector).magnitude();

            let accel2: Vector3 = Vector3 { 
                x: (velocity2 - *wind_vector).x*v12*drag_coefficient, 
                y: (velocity2 - *wind_vector).y*v12*drag_coefficient + grav, 
                z: (velocity2 - *wind_vector).z*v12*drag_coefficient 
            };

            let velocity3: Vector3 = self.velocity - accel2**dt;
            let v13: f64 = (velocity3 - *wind_vector).magnitude();

            let accel3: Vector3 = Vector3 { 
                x: (velocity3 - *wind_vector).x*v13*drag_coefficient, 
                y: (velocity3 - *wind_vector).y*v13*drag_coefficient + grav, 
                z: (velocity3 - *wind_vector).z*v13*drag_coefficient 
            };

            let velocity4: Vector3 = self.velocity - accel3**dt;
            let v14: f64 = (velocity4 - *wind_vector).magnitude();

            let accel4: Vector3 = Vector3 { 
                x: (velocity4 - *wind_vector).x*v14*drag_coefficient, 
                y: (velocity4 - *wind_vector).y*v14*drag_coefficient + grav, 
                z: (velocity4 - *wind_vector).z*v14*drag_coefficient 
            };

            let velocity5: Vector3 = self.velocity - accel4**dt;
            let v15: f64 = (velocity5 - *wind_vector).magnitude();

            let return_vel = self.velocity - (accel1 + accel2*2.0 + accel3*2.0 + accel4)**dt*(1.0/6.0 as f64);

            let retrun_pos = self.position + (self.velocity+return_vel)**dt;

            self.velocity = return_vel;
            self.position = retrun_pos;



        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::Vector3;
    

    #[test]
    fn verify_cross_product() {
        let x: Vector3 = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let y: Vector3 = Vector3 {x: 0.0, y: 1.0, z: 0.0};
        let z: Vector3 = Vector3 {x: 0.0, y: 0.0, z: 1.0};
        assert_eq!(x % y, z);
        assert_eq!(y % z, x);
        assert_eq!(z % x, y);
    }

    #[test]
    fn verify_dot_product() {
        let x: Vector3 = Vector3{x: 1.0, y: 0.0, z: 0.0};
        let y: Vector3 = Vector3{x: 0.0, y: 1.0, z: 0.0};
        let z: Vector3 = Vector3{x: 0.0, y: 0.0, z: 1.0};
        assert_eq!(x * y, 0.0);
        assert_eq!(y * z, 0.0);
        assert_eq!(x * z, 0.0);
    }


}