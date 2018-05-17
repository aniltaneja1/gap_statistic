#![allow(dead_code, unused)]  // TODO: Remove this when things settle into place.

use ndarray::{Array2, Array1};
use ndarray_rand::RandomExt;
use rand::distributions::Range;

// Centroid struct; hold place inside data
// TODO: Implement this
struct Centroid<'a> {
    data: &'a Array2<f64>
}

// Kmeans Entry point
// TODO: Implement this
fn kmeans<'a>(data: &'a Array2<f64>, k: u32, iter: u32, minit: &str) -> (Vec<Centroid<'a>>, Vec<u32>) {

    (vec![Centroid{data: &data}, Centroid{data: &data}], vec![1, 2])
}

// Obtain the optimal clusters for the given dataset.
pub fn optimal_k(data: Vec<Vec<f64>>, cluster_range: Vec<u32>) -> u32 {

    // Convert vector to Array
    // TODO: deal with this better than using unwrap, pass error if present
    let data = Array2::from_shape_vec(
    (data.len(), data[0].len()), data
    ).unwrap();

    5
}


// Calculate the gap value
fn calculate_gap(data: &Array2<f64>, n_clusters: u32) -> f64 {

    let n_refs = 5; // TODO: Add this as parameter
    let mut ref_dispersions = Array1::zeros((n_refs,));

    // For each reference check, run k-means on random data resembling shape of data.
    for i in 0..n_refs {

        // Generate some random data for this round
        let random_data = Array2::random(data.dim(), Range::new(0.0, 1.0));

        // Get centroids from data, each centroid contains .point() and .label()
        let centroids = kmeans(&random_data, i as u32, 10, "points");
        ref_dispersions[i] = 5;
    }

    5.6
}


// Calculate the dispersion
// TODO: Implement this
fn calculate_dispersion(data: &Array2<f64>, labels: Vec<u32>, centroids: Vec<Centroid>) -> f64 {
    1.1
}