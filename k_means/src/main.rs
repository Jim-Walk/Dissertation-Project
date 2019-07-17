use netcdf;

// Array is of width rows * cols, and of length rows
fn make_2d_float_array(rows: u64, cols: u64) -> Vec<Vec<f32>> {
    vec![vec![0.0; (rows * cols) as usize]; rows as usize]
}
fn make_2d_int_array(rows: u64, cols: u64) -> Vec<Vec<i32>> {
    vec![vec![0; (rows * cols) as usize]; rows as usize]
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

    let mut X = make_2d_float_array(samples_d.len, clusters_d.len);
    X[0] = x_var.get_float(true).unwrap();

    let mut GUESS = make_2d_int_array(repeat_d.len, clusters_d.len);
    GUESS[0] = guess_var.get_int(true).unwrap();

    println!("Reading data finished");
    println!("X[0] len {}", X[0].len());
    println!("X[1] len {}", X[1].len());

    let mut labels = vec![0; samples_d.len as usize];
    let mut labels_best = vec![0; samples_d.len as usize];

    let mut cluster_sizes = vec![0; clusters_d.len as usize];
    let mut old_cluster_centres = make_2d_float_array(clusters_d.len, features_d.len);
    let mut new_cluster_centres = make_2d_float_array(clusters_d.len, features_d.len);

    let mut inert_best = std::f32::MAX;
    for i_repeat in 0..repeat_d.len as usize {
        
        // Guess initial centers
        for k in 0..clusters_d.len as usize {
            let initial_idx = GUESS[i_repeat][k];
            for j in 0..features_d.len as usize {
                old_cluster_centres[k][j] = X[initial_idx as usize][j];
                new_cluster_centres[k][j] = 0.0; // this line may be unnecessary as data is already zeroed
            }
        }

        // Core k-mean stepping (Expectation-Maximization) begins here!
        let mut i_iter = 0;
        let mut dist_sum_new = 0.0;
        let TOL = 0.0001;
        let MAX_ITER = 100;
        loop {
            i_iter += 1;
            let dist_sum_old = dist_sum_new;
            dist_sum_new = 0.0;
            let mut k_best;

            // E-Step TODO Parallelise this!
            for i  in 0..samples_d.len as usize {
                k_best = 0;
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

            // M-Step first half
            for i in 0..samples_d.len as usize {
                k_best = labels[i];
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
            if i_iter == 1 || ((dist_sum_old - dist_sum_new > TOL) && i_iter < MAX_ITER){
                continue;
            } else {
                break;
            }
        }    

        if dist_sum_new < inert_best {
            inert_best = dist_sum_new;
            println!("lower");
            for i in 0..samples_d.len as usize {
                labels_best[i] = labels[i];
            }
        }
    }
    println!("Best inertia: {}", inert_best);
}
