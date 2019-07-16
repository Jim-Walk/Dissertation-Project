use netcdf;

fn make_2d_float_array(rows: u64, cols: u64) -> Vec<Vec<f32>> {
    let data = vec![0.0; (rows * cols) as usize];
    let array = vec![vec![5.0;5]; 5];
    array
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

    let p_x = x_var.get_float(true).unwrap();

    let p_guess = guess_var.get_int(true).unwrap();

    println!("Reading data finished");

    let labels = vec![0; samples_d.len as usize];
    let labels_best = vec![0; samples_d.len as usize];

    let cluster_sizes = vec![0; clusters_d.len];

    for k in 0..clusters_d.len as usize {
        let initial_idx = 
    }
}
