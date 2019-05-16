extern crate num;
use num::traits;
use std::ops::{AddAssign, DivAssign};
use std::any::Any;

pub struct RustStream<T: traits::Float>{
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Vec<T>,
    pub scalar: T,
}


impl <T> RustStream<T> where T: traits::Float + AddAssign<T> + num::Signed + DivAssign<T> + std::fmt::Display + Any{


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

    pub fn check_solution(&self, ntimes: i32, start_vals: [T; 3], arr_size: T, sum: T){
        let mut gold_a = start_vals[0];
        let mut gold_b = start_vals[1];
        let mut gold_c = start_vals[2];
        let mut gold_sum;

        // Do stream
        for _i in 0..ntimes{
            gold_c = gold_a;
            gold_b = self.scalar * gold_c;
            gold_c = gold_a + gold_b;
            gold_a = gold_b + self.scalar * gold_c;
        }
        // Do the reduction
        gold_sum = gold_a * gold_b * arr_size;

        // Calculate the average error
        let mut err_a = self.a.iter().fold(T::from(0).unwrap(), |sum, val| sum + num::abs(*val - gold_a));
        err_a /= T::from(self.a.len()).unwrap();

        let mut err_b = self.b.iter().fold(T::from(0).unwrap(), |sum, val| sum + num::abs(*val - gold_b));
        err_b /= T::from(self.b.len()).unwrap();

        let mut err_c = self.c.iter().fold(T::from(0).unwrap(), |sum, val| sum + num::abs(*val - gold_c));
        err_c /= T::from(self.c.len()).unwrap();

        let err_sum = num::abs(sum - gold_sum);

        let mut epsi = T::from(std::f64::EPSILON).unwrap();
        if Any::is::<f32>(&sum){
            epsi = T::from(std::f32::EPSILON).unwrap();
        }
        if err_a > epsi {
            println!("Error on a[]: {}", err_a)
        }
        if err_b > epsi {
            println!("Error on b[]: {}", err_b)
        }
        if err_c > epsi {
            println!("Error on c[]: {}", err_c)
        }
        if err_sum > T::from(1.0E-8).unwrap(){
            println!("error on sum")
        }
    }
}
