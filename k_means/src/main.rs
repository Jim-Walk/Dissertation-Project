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
            xsum = xsum + x[j];
            ysum = ysum + y[j];
            xysum = xysum + x[j] * y[j];
            xsqr_sum = xsqr_sum + x[j] * x[j];
            ysqr_sum = ysqr_sum + y[j] * y[j];
    }

    let num = (n_features * xysum) - (xsum * ysum);
    let deno = (n_features * xsqr_sum - xsum * xsum)* (n_features * ysqr_sum - ysum * ysum);
    num / deno.sqrt()
}

fn main() {
    let file = netcdf::open(&"data/SSWdata.nc").unwrap();
    let samples_d = file.root.dimensions.get("N_samples").unwrap();
    println!("Number of samples: {}", samples_d.len);

    let features_d = file.root.dimensions.get("N_features").unwrap();
    println!("Number of features: {}", features_d.len);
    
    let clusters_d = file.root.dimensions.get("N_clusters").unwrap();
    println!("Number of clusters: {}", clusters_d.len);
    
    let repeat_d = file.root.dimensions.get("N_repeat").unwrap();
    println!("Number of repeated runs: {}", repeat_d.len);

    let x_var = file.root.variables.get("X").unwrap();

    let guess_var = file.root.variables.get("GUESS").unwrap();

    let mut X = make_2d_float_array(samples_d.len, features_d.len, x_var.get_float(true).unwrap());

    let mut guess = make_2d_int_array(repeat_d.len, clusters_d.len, guess_var.get_int(true).unwrap());

    println!("Reading data finished");
    println!("X len {}", X.len());
    println!("X[0] len {}", X[0].len());
    println!("X[1] len {}", X[1].len());

    let mut labels = vec![0; samples_d.len as usize];
    let mut labels_best = vec![0; samples_d.len as usize];

    let mut cluster_sizes = vec![0; clusters_d.len as usize];
    let mut old_cluster_centres = vec![vec![0.0; (clusters_d.len * features_d.len) as usize]; clusters_d.len as usize];
    let mut new_cluster_centres = vec![vec![0.0; (clusters_d.len * features_d.len) as usize]; clusters_d.len as usize];

    let mut inert_best = std::f32::MAX;
    let mut e_timings = 0.0;
    for i_repeat in 0..repeat_d.len as usize {
        
        // Guess initial centers
        for k in 0..clusters_d.len as usize {
            let initial_idx = guess[i_repeat][k];
            for j in 0..features_d.len as usize {
                old_cluster_centres[k][j] = X[initial_idx as usize][j];
                new_cluster_centres[k][j] = 0.0; // this line may be unnecessary as data is already zeroed
            }
        }

        // Core k-mean stepping (Expectation-Maximization) begins here!
        let mut i_iter = 0;
        let mut dist_sum_new = 0.0;
        let TOL = 0.0001;
        let max_iter = 100;
        loop {
            i_iter += 1;
            let dist_sum_old = dist_sum_new;
            dist_sum_new = 0.0;

            // E-Step TODO Parallelise this!
            let t1 = Instant::now();
            for i in 0..samples_d.len as usize {
                let mut k_best = 0;
                let mut dist_min = correlation(features_d.len as f32, &X[i], &old_cluster_centres[k_best]);
                for k in 1..clusters_d.len as usize {
                    let mut dist = correlation(features_d.len as f32, &X[i], &old_cluster_centres[k]);
                    if dist < dist_min {
                        dist_min = dist;
                        k_best = k;
                    }
                }
                labels[i] = k_best;
                dist_sum_new += dist_min;
            }
            e_timings += (t1.elapsed().as_micros() as f64) / 1000.0;

            // M-Step first half
            for i in 0..samples_d.len as usize {
                let k_best = labels[i];
                cluster_sizes[k_best] += 1; // add one more point to this cluster
                // as the total number of smaples in each cluster is not known yet,
                // here we are just calulcating the sum, not the mean
                for j in 0..features_d.len as usize {
                    new_cluster_centres[k_best][j] += X[i][j];
                }
            }

            // M-step second half: convert sum to mean
            for k in 0..clusters_d.len as usize {
                for j in 0..features_d.len as usize {
                    if cluster_sizes[k] > 0 {
                        old_cluster_centres[k][j] = new_cluster_centres[k][j] / cluster_sizes[k] as f32;
                    }
                    new_cluster_centres[k][j] = 0.0;
                }
                cluster_sizes[k] = 0;
            }
            if i_iter == 1 || ((dist_sum_old - dist_sum_new > TOL) && i_iter < max_iter){
                continue;
            } else {
                break;
            }
        }    

        if dist_sum_new < inert_best {
            inert_best = dist_sum_new;
            for i in 0..samples_d.len as usize {
                labels_best[i] = labels[i];
            }
        }
    }
    println!("E-step time: {}", e_timings);
    println!("Best inertia: {}", inert_best);
}
