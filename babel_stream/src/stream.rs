extern crate num;
use num::traits;

//TODO figure out how to make this a vector of floats
pub struct RustStream<T: traits::Float>{
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Vec<T>
}


impl <T: traits::Float> RustStream <T>{
    pub fn triad(&mut self){
        println!("entered triad!");
        self.c[0] = self.a[0] + self.b[0];
    }
}
