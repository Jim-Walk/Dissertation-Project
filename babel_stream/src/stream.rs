//extern crate num;
extern crate rayon;
//use num::traits::Float;
use std::ops::{AddAssign, DivAssign};
use std::any::Any;
use rayon::prelude::*;
//use stream;
use num::Float;

pub struct RustStream<T: Float + Send + Sync + std::iter::Sum>{
    pub a: Vec<T>,
    pub b: Vec<T>,
    pub c: Vec<T>,
    pub scalar: T,
}


impl <T> RustStream<T> 
where T: Float + AddAssign<T> + num::Signed + DivAssign<T> + std::fmt::Display + Any + Send + Sync + std::iter::Sum,
[T]: Send + Sync //+ std::marker::Sized
{
    pub fn copy(&mut self){
       // for (c_i, a_i) in self.c.par_iter().zip(self.a.iter()){
       //     *c_i = *a_i;
       // }
       //self.c.par_iter_mut()
       //     .zip(self.a.par_iter())
       //     .for_each(|(c, a)| *c = *a);
       self.c.par_chunks_mut(1000)
            .zip(self.a.par_chunks(1000))
            .for_each(|(c, a)| c.copy_from_slice(a));
    }

    pub fn mul(&mut self){
        //for (b_i, c_i) in self.b.iter_mut().zip(self.c.iter()){
        //    *b_i = self.scalar * *c_i;
        //}
        // This scalar variable is needed as self is mutably borrowd by function call
        let scalar_imut = self.scalar;
        self.b.par_iter_mut()
            .zip(self.c.par_iter())
            .for_each(|(b, c)| *b = scalar_imut * *c);
    }

    pub fn add(&mut self){
        //for ((c_i, a_i), b_i) in self.c.iter_mut().zip(self.a.iter()).zip(self.b.iter()){
        //    *c_i = *a_i + *b_i;
        //}
        self.c.par_iter_mut()
            .zip(self.b.par_iter())
            .zip(self.a.par_iter())
            .for_each(|((c, b), a)| *c = *a + *b);
    }

    pub fn triad(&mut self){
        //TODO serial works but parallel causes problems
        //for ((a_i, c_i), b_i) in self.a.iter_mut().zip(self.c.iter()).zip(self.b.iter()){
        //    *a_i = *b_i + self.scalar * *c_i;
        //}
        let scalar_imut = self.scalar;
        self.a.par_iter_mut()
            .zip(self.c.par_iter())
            .zip(self.b.par_iter())
            .for_each(|((a, c), b)| *a = *b + scalar_imut + *c);
    }

    pub fn dot(&mut self)->T{
        let sum1: T = T::from(0).unwrap();
        //for (a_i, b_i) in self.a.iter().zip(self.b.iter()){
        //    sum += *a_i * *b_i;
        //}
        let ret_sum  = self.a.par_iter()
                            .zip(self.b.par_iter())
                            .fold(|| sum1, |acc, it| acc + *it.0 * *it.1).sum();
                            //.reduce(|| (&sum1, &sum2), |a, b| (*a.0 * *a.1, *b.0 * *b.0));
        ret_sum
    }

    pub fn check_solution(&self, ntimes: i32, start_vals: [T; 3], arr_size: T, sum: T){
        let mut gold_a = start_vals[0];
        let mut gold_b = start_vals[1];
        let mut gold_c = start_vals[2];

        // Do stream
        for _i in 0..ntimes{
            gold_c = gold_a;
            gold_b = self.scalar * gold_c;
            gold_c = gold_a + gold_b;
            gold_a = gold_b + self.scalar * gold_c;
        }
        // Do the reduction
        let gold_sum = gold_a * gold_b * arr_size;

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
            println!("error on sum: {} \nExpected {} found {}", err_sum, gold_sum, sum)
        }
    }
}
