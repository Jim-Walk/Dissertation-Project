extern crate num;
use num::traits;

pub struct RustStream<T: traits::Float>{
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Vec<T>,
    pub scalar: T
}


impl <T: traits::Float> RustStream <T>{

    pub fn run(&mut self, num_times: i32){
        let mut sum: T;
        for i in 0..num_times{
            self.triad();
        }
        sum = self.dot();
    }

    pub fn dot(&mut self)->T{
        let mut sum: T;
        sum = 0 as T;
        for i in 0..100{
            sum = self.a[i] * self.b[i]; 
        }
        sum
    }

    pub fn triad(&mut self){
        println!("entered triad!");
        self.c[0] = self.a[0] + self.b[0];
    }
}
