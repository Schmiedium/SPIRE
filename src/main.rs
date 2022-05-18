mod Vector3 {
    pub struct Vector3 {
        x: f64,
        y: f64,
        z: f64,
    }

    pub fn magnitude (vec : &Vector3) -> f64 {
        return f64::sqrt(vec.x*vec.x + vec.y*vec.y + vec.z*vec.z);
    }
}


fn main() {
    
}
