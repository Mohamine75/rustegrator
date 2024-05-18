// This module provides utilities for generating directed acyclic graphs (DAGs)
// with varying connectivity properties.
// It uses the `rand` crate for randomness, which facilitates the generation of random graphs.


use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

/// Generates a directed acyclic graph (DAG) where each node has exactly one outgoing edge,
/// except for the last node which has none. This results in a simple linear chain of nodes.
///
/// # Arguments
///
/// * `max_size` - The number of nodes in the generated graph.
///
/// # Returns
///
/// A `Vec<Vec<i32>>` representing the adjacency matrix of the DAG.
///
/// # Examples
///
/// ```
/// let matrix = one_arity_matrix_generator(5);
/// assert_eq!(matrix.len(), 5);
/// ```
pub fn one_arity_matrix_generator(max_size: usize) -> Vec<Vec<i32>> {
    let mut matrice = vec![vec![0; max_size]; max_size]; // Initialize the adjacency matrix.
    let mut rng = thread_rng(); // Create a random number generator.

    let mut available_nodes: Vec<usize> = (0..max_size).collect();
    available_nodes.shuffle(&mut rng); // Shuffle the array of available nodes.

    // Connect each node to the next in the shuffled order, forming a simple path.
    for i in 0..max_size - 1 {
        matrice[available_nodes[i]][available_nodes[i + 1]] = 1;
    }

    matrice
}


/// Adds a new node to an existing directed acyclic graph and randomly creates an edge
/// from one of the existing nodes to this new node.
///
/// # Arguments
///
/// * `matrice` - A mutable reference to the adjacency matrix of the existing graph.
///
/// # Examples
///
/// ```
/// let mut matrix = vec![vec![0; 3]; 3]; // A 3x3 matrix
/// add_node(&mut matrix);
/// assert_eq!(matrix.len(), 4); // The matrix should now be 4x4
/// ```
pub fn add_node(matrice: &mut Vec<Vec<i32>>) {
    let mut rng = thread_rng(); // Create a random number generator.
    let size = matrice.len();

    // Extend each existing row by one column, initializing with 0.
    for row in matrice.iter_mut() {
        row.push(0);
    }

    // Add a new row at the bottom, initialized to 0s, representing no outgoing edges from the new node.
    matrice.push(vec![0; size + 1]);

    // Select a random row index from the existing nodes to create an edge to the new node.
    let random_row_index = rng.gen_range(0..size);
    matrice[random_row_index][size] = 1;
}


/// Generates a directed acyclic graph (DAG) with a random connectivity pattern.
/// Starts with a single node and repeatedly adds new nodes using `add_node`,
/// resulting in a graph where node connectivity is randomly determined.
///
/// # Arguments
///
/// * `size` - The desired number of nodes in the graph.
///
/// # Returns
///
/// A `Vec<Vec<i32>>` representing the adjacency matrix of the randomly generated DAG.
///
/// # Examples
///
/// ```
/// let matrix = random_arity_matrix_generator(5);
/// assert_eq!(matrix.len(), 5); // The matrix should have 5 rows
/// ```
pub fn random_arity_matrix_generator(size: usize) -> Vec<Vec<i32>> {
    let mut matrice_res = vec![vec![0]]; // begins with a 1x1 matrix
    // Iteratively add nodes until the matrix reaches the desired size.

    for _ in 1..size {
        add_node(&mut matrice_res);
    }

    matrice_res
}
