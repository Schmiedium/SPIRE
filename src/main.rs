mod Vector3 {
    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vector3{
        pub fn magnitude (&self) -> f64 {
            return f64::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
        }

        pub fn cross_product(vec1: &Vector3, vec2: &Vector3) -> Vector3 {
            let x = vec1.y*vec2.z - vec1.z*vec1.y;
            let y = vec1.z*vec1.x - vec1.x*vec2.z;
            let z = vec1.x*vec2.y - vec1.y*vec2.x;
            let cross = Vector3 {x, y, z};
            return cross;
        }

        pub fn dot_product(&self, vec2: &Vector3) -> f64 {
            return self.x*vec2.x + self.y*vec2.y + self.z*vec2.z;
        }
    }



}


fn main() {
    let test: Vector3::Vector3 = Vector3::Vector3{x: 12.9, y: 57.3, z: 82.9};
    println!("magnitude of test: {}", test.magnitude());
}
