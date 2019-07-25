use netcdf;
use std::time::Instant;
use rayon::prelude::*;

// Array is of width rows * cols, and of length rows
fn make_2d_float_array(rows: u64, cols: u64, data: Vec<f32>) -> Vec<Vec<f32>> {
    let mut v = vec![vec![0.0; 1]; rows as usize];
    for i in 0..rows as usize {
        let lo = cols as usize * i;
        let hi = cols as usize * (i+1);
        v[i] = data[lo..hi].to_vec();
    }
    v
}
fn make_2d_int_array(rows: u64, cols: u64, data: Vec<i32>) -> Vec<Vec<i32>> {
    vec![vec![0; 1]; rows as usize];;
    let mut v = vec![vec![0; 1]; rows as usize];
    for i in 0..rows as usize {
        let lo = cols as usize * i;
        let hi = cols as usize * (i+1);
        v[i] = data[lo..hi].to_vec();
    }
    v
}

fn correlation(n_features: f32, x: &Vec<f32>, y: &Vec<f32>)-> f32{
    let (mut xsum, mut ysum, mut xysum, mut xsqr_sum, mut ysqr_sum) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for j in 0..n_features as usize {
            xsum += x[j];
            ysum += y[j];
            xysum += x[j] * y[j];
            xsqr_sum += x[j] * x[j];
            ysqr_sum += y[j] * y[j];
    }

    let num = (n_features * xysum) - (xsum * ysum);
    let deno = (n_features * xsqr_sum - xsum * xsum)* (n_features * ysqr_sum - ysum * ysum);
    num / deno.sqrt()
}

fn main() {
    let file = netcdf::open(&"data/SSWdata.nc").unwrap();
    let samples_d = &file.root.dimensions["N_samples"];
    println!("Number of samples: {}", samples_d.len);

    let features_d = &file.root.dimensions["N_features"];
    println!("Number of features: {}", features_d.len);
    
    let clusters_d = &file.root.dimensions["N_clusters"];
    println!("Number of clusters: {}", clusters_d.len);
    
    let repeat_d = &file.root.dimensions["N_repeat"];
    println!("Number of repeated runs: {}", repeat_d.len);

    let x_var = &file.root.variables["X"];

    let guess_var = &file.root.variables["GUESS"];

    let X = make_2d_float_array(samples_d.len, features_d.len, x_var.get_float(true).unwrap());

    let guess = make_2d_int_array(repeat_d.len, clusters_d.len, guess_var.get_int(true).unwrap());

    println!("=====reading data finished======");

    let mut labels = vec![0; samples_d.len as usize];
    let mut labels_best = vec![0; samples_d.len as usize];

    let mut cluster_sizes = vec![0; clusters_d.len as usize];
    let mut old_cluster_centres = vec![vec![0.0; (clusters_d.len * features_d.len) as usize]; clusters_d.len as usize];
    let mut new_cluster_centres = vec![vec![0.0; (clusters_d.len * features_d.len) as usize]; clusters_d.len as usize];

    let mut inert_best = std::f32::MAX;
    let mut e_timings = 0.0;
    let mut m1_timings = 0.0;
    let mut m2_timings = 0.0;
    let t0 = Instant::now();
    println!("=====Applying K-mean======");
    for i_repeat in 0..repeat_d.len as usize {
        
        // Guess initial centers
        for k in 0..clusters_d.len as usize {
            let initial_idx = guess[i_repeat][k];
            for j in 0..features_d.len as usize {
                old_cluster_centres[k][j] = X[initial_idx as usize][j];
            }
        

        // Core k-mean stepping (Expectation-Maximization) begins here!
        let mut i_iter = 0;
        let mut dist_sum_new = 0.0;
        let tol = 0.0001;
        let max_iter = 100;
        loop {
            i_iter += 1;
            let dist_sum_old = dist_sum_new;
            dist_sum_new = 0.0;

            // E-Step TODO Parallelise this!
            let t1 = Instant::now();

            for (idx, item) in X.iter().enumerate() {
                let mut k_best = 0;
                let mut dist_min = correlation(features_d.len as f32, &item, &old_cluster_centres[k_best]);
                for k in 1..clusters_d.len as usize {
                    let dist = correlation(features_d.len as f32, &item, &old_cluster_centres[k]);
                    if dist < dist_min {
                        dist_min = dist;
                        k_best = k;
                    }
                }
                labels[idx] = k_best;
                dist_sum_new += dist_min;
            }

            e_timings += t1.elapsed().as_micros() as f64 / 1000.0;

            // M-Step first half
            let t2 = Instant::now();
            for i in 0..samples_d.len as usize {
                let k_best = labels[i];
                cluster_sizes[k_best] += 1; // add one more point to this cluster
                // as the total number of smaples in each cluster is not known yet,
                // here we are just calulcating the sum, not the mean
                for j in 0..features_d.len as usize {
                    new_cluster_centres[k_best][j] += X[i][j];
                }
            }
            m1_timings += t2.elapsed().as_micros() as f64 / 1000.0;

            // M-step second half: convert sum to mean
            let t3 = Instant::now();
            for k in 0..clusters_d.len as usize {
                for j in 0..features_d.len as usize {
                    if cluster_sizes[k] > 0 {
                        old_cluster_centres[k][j] = new_cluster_centres[k][j] / cluster_sizes[k] as f32;
                    }
                    new_cluster_centres[k][j] = 0.0;
                }
                cluster_sizes[k] = 0;
            }
            m2_timings += t3.elapsed().as_micros() as f64 / 1000.0;
            if i_iter == 1 || ((dist_sum_old - dist_sum_new > tol) && i_iter < max_iter){
                continue;
            } else {
                break;
            }
        }    

        if dist_sum_new < inert_best {
            inert_best = dist_sum_new;
            labels_best[..samples_d.len as usize].clone_from_slice(&labels[..samples_d.len as usize]);
        }
    }
    let a_timings = t0.elapsed().as_micros() as f64 / 1000.0;

    // write data back to files 

    let mut file = netcdf::append("data/SSWdata.nc").unwrap();

    let mut inert_c = file.root.variables.get_mut("INERT_C").unwrap();
    inert_c.put_value_at(inert_best, &[0]);

    let mut y_c = file.root.variables.get_mut("Y_C").unwrap();
    let labels_w: Vec<i32>= labels_best.iter().map(|e| *e as i32).collect();
    y_c.put_values_at(&labels_w, &[0], &[samples_d.len as usize]);


    println!("==== Finished Writing Data ====");
    println!("Best inertia: {}", inert_best);
    println!("Kmean total time use (ms) {}", a_timings);
    println!("E-step time (ms): {}", e_timings);
    println!("M-step-1st half time (ms): {}", m1_timings);
    println!("M-step-2nd half time (ms): {}", m2_timings);
}
