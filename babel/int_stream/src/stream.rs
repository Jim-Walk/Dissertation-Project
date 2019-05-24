//extern crate num;
extern crate rayon;
use rayon::prelude::*;
//use stream;

pub struct RustStream{
    pub a: Vec<i32>,
    pub b: Vec<i32>,
    pub c: Vec<i32>,
    pub scalar: i32,
}


impl RustStream
{
    //pub fn init_arrays(&mut self, arr_size: T){
    //    
    //    if Any::is::<f32>(&arr_size){
    //        let size = f64::from(arr_size) as f32;
    //        let zero = 0f32;
    //        let new_a: Vec<_> = (zero..size).into_par_iter()
    //                                        .map(|mut foo| foo = 8.0) //T::from(0.1).unwrap())
    //                                        .collect();
    //    }
    //}
    pub fn copy(&mut self){
       self.c.par_chunks_mut(1000)
            .zip(self.a.par_chunks(1000))
            .for_each(|(c, a)| c.copy_from_slice(a));
    }

    pub fn mul(&mut self){
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
        //for ((a_i, c_i), b_i) in self.a.iter_mut().zip(self.c.iter()).zip(self.b.iter()){
        //    *a_i = *b_i + self.scalar * *c_i;
        //}
        let scalar_imut = self.scalar;
        self.a.par_iter_mut()
            .zip(self.c.par_iter())
            .zip(self.b.par_iter())
            .for_each(|((a, c), b)| *a = *b + scalar_imut * *c);
    }

    pub fn dot(&mut self)->i32{
        //for (a_i, b_i) in self.a.iter().zip(self.b.iter()){
        //    sum += *a_i * *b_i;
        //}
        let ret_sum  = self.a.par_iter()
                            .zip(self.b.par_iter())
                            .fold(|| 0, |acc, it| acc + *it.0 * *it.1).sum();
                            //.reduce(|| (&sum1, &sum2), |a, b| (*a.0 * *a.1, *b.0 * *b.0));
        ret_sum
    }

    pub fn check_solution(&self, ntimes: i32, start_vals: [i32; 3], arr_size: f32, sum: i32){
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
        let gold_sum = gold_a * gold_b * arr_size as i32;

        // Calculate the average error
        let mut err_a = self.a.iter().fold(0, |sum, val| sum + num::abs(*val - gold_a));
        err_a /= self.a.len() as i32;

        let mut err_b = self.b.iter().fold(0, |sum, val| sum + num::abs(*val - gold_b));
        err_b /= self.b.len() as i32;

        let mut err_c = self.c.iter().fold(0, |sum, val| sum + num::abs(*val - gold_c));
        err_c /= self.c.len() as i32;

        let err_sum = num::abs(sum - gold_sum);

        let mut epsi = 0;
        if err_a > epsi {
            println!("Error on a[]: {}", err_a)
        }
        if err_b > epsi {
            println!("Error on b[]: {}", err_b)
        }
        if err_c > epsi {
            println!("Error on c[]: {}", err_c)
        }
        //if err_sum > (1.0E-8).unwrap(){
        //    println!("error on sum: {} \nExpected {} found {}", err_sum, gold_sum, sum)
        //}
    }
}
