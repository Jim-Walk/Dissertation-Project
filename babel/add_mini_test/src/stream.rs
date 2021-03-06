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
    pub chunk_size: usize
}


impl <T> RustStream<T> 
where T: Float + AddAssign<T> + num::Signed + DivAssign<T> + std::fmt::Display + Any + Send + Sync + std::iter::Sum,
[T]: Send + Sync, f64: std::convert::From<T>
{
    pub fn init_arrays(&mut self, arr_size: usize){
        vec![0.0; arr_size].par_iter()
                        .map(|_| T::from(0.1).unwrap())
                        .collect_into_vec(&mut self.a);
        vec![0.0; arr_size].par_iter()
                        .map(|_| T::from(0.2).unwrap())
                        .collect_into_vec(&mut self.b);
        vec![0.0; arr_size].par_iter()
                        .map(|_| T::from(0.0).unwrap())
                        .collect_into_vec(&mut self.c);
    }
    pub fn copy(&mut self){
       self.c.par_chunks_mut(1000)
            .zip(self.a.par_chunks(1000))
            .for_each(|(c, a)| c.copy_from_slice(a));
    }

    pub fn mul(&mut self){
        // This scalar variable is needed as self is mutably borrowd by function call
        let scalar_imut = self.scalar;
        self.b.par_chunks_mut(self.chunk_size)
            .zip(self.c.par_chunks_mut(self.chunk_size))
            .for_each(|(b, c)| 
                            for (b_i, c_i) in b.iter_mut()
                                                .zip(c.iter()){
                                                    *b_i = scalar_imut * *c_i 
                            });
    }

    pub fn add(&mut self){
        self.c.par_chunks_mut(self.chunk_size)
            .zip(self.b.par_chunks(self.chunk_size))
            .zip(self.a.par_chunks(self.chunk_size))
            .for_each(|((c, b), a)| 
                            for ((c_i, b_i), a_i) in c.iter_mut()
                                                        .zip(b.iter())
                                                        .zip(a.iter()){
                                                            *c_i = *a_i + *b_i
                                                        });
    }

    pub fn triad(&mut self){
        let scalar_imut = self.scalar;
        self.a.par_chunks_mut(self.chunk_size)
            .zip(self.c.par_chunks(self.chunk_size))
            .zip(self.b.par_chunks(self.chunk_size))
            .for_each(|((a, c), b)|
                            for (( a_i, c_i,), b_i) in a.iter_mut()
                                                        .zip(c.iter())
                                                        .zip(b.iter()){
                                                            *a_i = *b_i + scalar_imut * *c_i
                                                        });
    }

    pub fn dot(&mut self)->T{
        
        self.a.par_chunks(self.chunk_size)
            .zip(self.b.par_chunks(self.chunk_size))
            .map(|(a,b)| a.iter().zip(b.iter())
                      .fold(T::from(0).unwrap(), | acc, ele| acc + *ele.0 * *ele.1)
                      ).sum()
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
