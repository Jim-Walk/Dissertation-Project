extern crate num;
use num::traits;
use std::ops::AddAssign;

pub struct RustStream<T: traits::Float>{
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Vec<T>,
    pub scalar: T,
}


impl <T> RustStream<T> where T: traits::Float + AddAssign<T> {

    pub fn run(&mut self, num_times: i32){
        let mut sum: T;
        for i in 0..num_times{
            self.copy();
            self.mul();
            self.add();
            self.triad();
            sum = self.dot();
        }
    }

    pub fn copy(&mut self){
        for i in 0..self.a.len(){
            self.c[i] = self.a[i];
        }
    }

    pub fn mul(&mut self){
        for i in 0..self.b.len(){
            self.b[i] = self.scalar * self.c[i];
        }
    }

    pub fn add(&mut self){
        for i in 0..self.a.len(){
            self.c[i] = self.a[i]+self.b[i];
        }
    }

    pub fn dot(&mut self)->T{
        let mut sum: T = T::from(0).unwrap();
        for i in 0..self.a.len(){
            sum += self.a[i] * self.b[i]; 
        }
        sum
    }

    pub fn triad(&mut self){
        for i in 0..self.a.len(){
            self.a[i] = self.b[i] + self.scalar + self.c[i];
    }
}
