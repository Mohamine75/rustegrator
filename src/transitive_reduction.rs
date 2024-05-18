use std::collections::{HashMap, VecDeque};

/// Performs a topological reduction based on a given topological order of a graph.
/// It identifies and marks the edges that can be removed to achieve transitive reduction.
///
/// # Arguments
///
/// * `ordre_topologique` - A reference to a vector of tuples representing the topological order of the graph.
///
/// # Returns
///
/// A vector of tuples representing the edges to be removed to achieve a transitive reduction.
pub fn transitive_reduction_topologique(ordre_topologique: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut to_remove = Vec::new();
    let n = ordre_topologique.len();
    let mut mark = vec![false; n];

    // Create a mapping from vertex to its index in the topological sort.
    let vertex_to_index: HashMap<usize, usize> = ordre_topologique.iter().enumerate()
        .map(|(i, &(src, _))| (src, i))
        .collect();

    for (i, &(_, target)) in ordre_topologique.iter().enumerate() {
        for j in 0..i {
            mark[j] = false;  // Reset marks for this run
        }

        // Mark predecessors
        for j in 0..i {
            let (source, intermediate) = ordre_topologique[j];
            if intermediate == target && !mark[j] {
                to_remove.push((source, target));  // Mark this edge for removal
                mark[j] = true;
            }
        }
    }

    ordre_topologique.iter().cloned()
        .filter(|&(src, tgt)| !to_remove.contains(&(src, tgt)))
        .collect()
}

/// Finds the largest index j for which the vertex connects to a specified vertex in the topological order.
///
/// # Arguments
///
/// * `ordre_topologique` - A reference to the topological order of the graph.
/// * `vertex` - The vertex for which to find the maximum index.
///
/// # Returns
///
/// The maximum index of vertices connecting to the specified vertex.
fn trouver_jmax(ordre_topologique: &Vec<(usize, usize)>, vertex: usize) -> usize {
    for (i, &item) in ordre_topologique.iter().enumerate().rev() {
        if item.1 == vertex {
            return item.0;
        }
    }
    0
}


/// Applies the transitive reduction to a graph given the edges to remove.
///
/// # Arguments
///
/// * `length` - The number of vertices in the graph.
/// * `to_remove` - A vector of tuples representing edges to be removed.
///
/// # Returns
///
/// An adjacency matrix representing the graph after applying the transitive reduction.
pub fn apply_transitive_reduction(length: usize, to_remove: Vec<(usize, usize)>) -> Vec<Vec<i32>> {
    let mut matrice = vec![vec![0; length]; length];
    for (source, target) in to_remove {
        matrice[source][target] = 1;
    }
    matrice
}

/// Generates a topological order for a graph represented by an adjacency matrix.
///
/// # Arguments
///
/// * `matrice` - An adjacency matrix of the graph.
///
/// # Returns
///
/// A vector of tuples representing the topological order of the graph.
pub fn create_ordre_topologique(matrice: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let n = matrice.len();
    let mut in_degree = vec![0; n];
    let mut queue = VecDeque::new();
    let mut ordre_topologique = Vec::new();

    // Calculate in-degrees of each vertex.
    for i in 0..n {
        for j in 0..n {
            if matrice[i][j] == 1 {
                in_degree[j] += 1;
            }
        }
    }

    // Enqueue vertices with zero in-degree.
    for i in 0..n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    // Kahn's algorithm for topological sorting.
    while let Some(i) = queue.pop_front() {
        for j in 0..n {
            if matrice[i][j] == 1 {
                ordre_topologique.push((i, j));
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    queue.push_back(j);
                }
            }
        }
    }

    ordre_topologique
}




/// Converts a topological order back to an adjacency matrix.
///
/// # Arguments
///
/// * `ordre_topologique` - A vector of tuples representing the topological order.
/// * `nombre_noeuds` - The number of nodes in the graph.
///
/// # Returns
///
/// An adjacency matrix representing the graph based on the topological order.
pub fn ordre_topologique_to_matrice_adj(ordre_topologique: Vec<(usize, usize)>, nombre_noeuds: usize) -> Vec<Vec<i32>> {
    let mut matrice = vec![vec![0; nombre_noeuds]; nombre_noeuds];

    for (source, target) in ordre_topologique {
        if source < nombre_noeuds && target < nombre_noeuds {
            matrice[source][target] = 1;
        }
    }

    matrice
}

/// Retrieves a list of unique order indices from a list of tuples representing edges.
///
/// # Arguments
///
/// * `matrice` - A vector of tuples representing edges.
///
/// # Returns
///
/// A vector of unique vertex indices in their order of appearance.
fn get_ordre(matrice: Vec<(usize, usize)>) -> Vec<usize> {
    let mut order = Vec::new();
    let mut cpt = 0;
    for nombre in matrice  {
        if cpt>0 {
            if order[cpt-1]!= nombre.0 {
                order.push(nombre.0);
                cpt+=1;
            }
        }else{
            order.push(nombre.0);
            cpt+=1;
        }

    }
     order
}



/// Checks if there is a direct edge from a specific vertex to another in the topological order.
///
/// # Arguments
///
/// * `ordre_topologique` - A reference to the topological order of the graph.
/// * `vertex` - The starting vertex.
///
/// # Returns
///
/// `true` if there is a direct edge from the specified vertex, otherwise `false`.

fn edge_to_vertex(ordre_topologique: &[(usize, usize)], vertex: usize) -> bool {
    ordre_topologique.iter().any(|&(_, target)| target == vertex)
}


/// Checks if there is a direct connection from one vertex to another in the topological order.
///
/// # Arguments
///
/// * `ordre_topologique` - A reference to the vector of tuples representing the topological order.
/// * `vertexj` - The starting vertex.
/// * `vertexi` - The ending vertex.
///
/// # Returns
///
/// `true` if there is a direct connection from `vertexj` to `vertexi`, otherwise `false`.
fn vertex_to_vertex (ordre_topologique: &Vec<(usize, usize)>,vertexj : usize,vertexi : usize) -> bool {
    for i in 0..ordre_topologique.len(){
        if ordre_topologique[i].0 == vertexj && ordre_topologique[i].1 == vertexi  {
            return true
        }
    }
    false
}

/// Applies a Floyd-Warshall based transitive reduction algorithm on a given matrix.
///
/// # Arguments
/// * `matrice` - A vector of vector of i32s representing the adjacency matrix to reduce.
///
/// # Returns
/// The transitive reduced matrix.
pub fn transitive_reduction_using_floyd_warshall(matrice: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = matrice.len();
    let mut reduction = matrice.clone(); // Work on a clone to avoid altering the original during iteration

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if reduction[i][k] == 1 && reduction[k][j] == 1 {
                    reduction[i][j] = 0; // Remove the direct edge if an indirect path exists
                }
            }
        }
    }

    reduction
}






