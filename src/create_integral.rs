use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use rand::seq::SliceRandom;
use regex::Regex;
use crate::generator_matrix::one_arity_matrix_generator;
use crate::generator_matrix::random_arity_matrix_generator;
use crate::resolution_using_lists::list_BIT_resolution;
use crate::resolution_using_hashmap::Graph;
mod transitive_reduction;
mod generator_matrix;
mod resolution_using_lists;
mod resolution_using_hashmap;

/// Displays a matrix in a readable format.
/// Each row of the matrix is displayed in brackets, separated by commas.
fn afficher_matrice(matrice: Vec<Vec<i32>>) {
    let lignes_affichees: Vec<String> = matrice.iter()
        .map(|ligne| {
            format!("[{}]", ligne.iter()
                .map(|&val| val.to_string())
                .collect::<Vec<String>>()
                .join(", "))
        })
        .collect();

    println!("[{}]", lignes_affichees.join(", "));
}


/// Verifies if all elements in a matrix are zero, indicating resolution of all nodes.
/// Returns true if all nodes are resolved, false otherwise.
fn verify_all_zero(matrice: Vec<Vec<i32>>) -> bool {
    for ligne in matrice {
        for valeur in ligne {
            if valeur == 1 {
                return false;
            }
        }
    }
    return true;
}


/// Reads a matrix from a specified file path.
/// Returns a vector of vectors of integers representing the matrix.
fn lire_matrice_à_partir_du_fichier(chemin: &str) -> Vec<Vec<i32>> {
    let path = Path::new(chemin);
    let file = File::open(path).expect("Cannot open the file");
    let buf = io::BufReader::new(file);

    let mut matrice = Vec::new();

    for line in buf.lines() {
        let line = line.expect("Error during the reading of a line");
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().expect("Error during the conversion to an integer"))
            .collect();
        matrice.push(nums);
    }

    matrice
}




/// Constructs an adjacency list from a matrix.
fn construire_adj(matrice: Vec<Vec<i32>>) -> HashMap<usize, Vec<usize>> {
    let mut adj = HashMap::new();
    for (i, ligne) in matrice.iter().enumerate() {
        let mut successeurs = Vec::new();
        for (j, &valeur) in ligne.iter().enumerate() {
            if valeur == 1 {
                successeurs.push(1);
            } else {
                successeurs.push(0);
            }
        }
        adj.insert(i, successeurs);
    }
    adj
}

/// Reads adjacency data from a file and returns a tuple containing the adjacency list and its size.
fn lire_adjacence_a_partir_du_fichier(chemin: &str) -> (HashMap<usize, Vec<usize>>, usize) {
    let path = Path::new(chemin);
    let file = File::open(path).expect("Cannot open the file");
    let buf = io::BufReader::new(file);
    let mut size = 0;
    let mut adj = HashMap::new();

    for (index, line) in buf.lines().enumerate() {
        let line = line.expect("Error during the reading of a line");
        let nums: Vec<usize> = line.split_whitespace()
            .map(|num| num.parse::<usize>().expect("Error during the conversion to an integer"))
            .collect();
        size = nums.len();
        adj.insert(index, nums);
    }

    (adj, size)
}

/// Constructs a predecessor list from an adjacency list.
fn construire_pred(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>> {
    let mut pred = HashMap::new();

    // We suppose that every from 0 to adj.len()-1 are present in adj, because squared matrix and function build adj
    for (node, successors) in adj {
        for (i, &succ) in successors.iter().enumerate() {
            if succ == 1 { // if succ is 1, then 'node' is predecessor of 'i'
                pred.entry(i).or_insert_with(Vec::new).push(*node);
            }
        }
    }
    // Ensure every vertex has an entry even if it has no predecessors
    for node in 0..adj.len() {
        pred.entry(node).or_insert_with(Vec::new);
    }
    pred
}

/// Checks if all indices from 0 to `taille-1` are present in a given string.
fn verifier_indices(chaine: &str, taille: usize) -> bool {
    let re = Regex::new(r"1 (\d+)").unwrap();
    let mut indices = HashSet::new();
    for cap in re.captures_iter(chaine) {
        if let Some(matched) = cap.get(1) {
            if let Ok(num) = matched.as_str().parse::<usize>() {
                indices.insert(num);
            }
        }
    }
    (0..taille).all(|i| indices.contains(&i))
}


fn main() {

    let file_path = "resultsListes.txt";
    let file_path2 = "resultsHashmap.txt";
    let file_path3 = "comparaisonResult.txt";
    let integrales_path = "integrales.txt";

    let file = File::create(file_path).expect("Unable to create file");
    let file2 = File::create(file_path2).expect("Unable to create file");
    let file3 = File::create(file_path3).expect("Unable to create file");
    let integrales = File::create(integrales_path).expect("Unable to create file");

    let mut file = BufWriter::new(file);
    let mut file2 = BufWriter::new(file2);
    let mut file3 = BufWriter::new(file3);
    let mut integrales = BufWriter::new(integrales);

    let testMatricehasard = one_arity_matrix_generator(6);
    println!("{:?}", testMatricehasard);

    for size in 5..=150 {
        let matrix = one_arity_matrix_generator(size);
        // Pour générer des matrices d'arite plus complexe
        //let matrix = random_arity_matrix_generator(size);
        let matrix2 = matrix.clone();
        let adj = construire_adj(matrix2.clone());
        let pred = construire_pred(&adj);
        let mut g = Graph::new(adj, pred, matrix2.len());
        let start_time = Instant::now();
        let _results = list_BIT_resolution(matrix);
        let duration = start_time.elapsed();

        let start_time2 = Instant::now();
        let _results2 = g.resolution_adjacence();
        let duration2 = start_time2.elapsed();

        // Comparaison des résultats
        let compar = _results == _results2;

        // Écriture des résultats dans le fichier
        writeln!(file, "{} {:?}", size, duration)
            .expect("Unable to write to file");
        writeln!(file2, "{} {:?}", size, duration2)
            .expect("Unable to write to file");

        // Écriture de true ou false dans le fichier 3
        writeln!(file3, "Comparaison: {}", compar)
            .expect("Unable to write to file");
        writeln!(integrales, "{}", _results)
            .expect("Unable to write to file");

        println!("Comparaison: {}", compar);
    }
    file.flush().expect("Unable to flush file");
    file2.flush().expect("Unable to flush file");
    file3.flush().expect("Unable to flush file");
    integrales.flush().expect("Unable to flush file");
}