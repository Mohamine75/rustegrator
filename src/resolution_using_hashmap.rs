// This module provides utilities for working with graphs represented as adjacency and predecessor lists using hash maps.
// It supports operations such as adjacency resolution, transitive reduction, and displaying the adjacency matrix.

use std::collections::HashMap;
use crate::construire_pred;
use crate::transitive_reduction::{create_ordre_topologique, transitive_reduction_topologique};


/// Represents a directed graph with adjacency and predecessor lists.
pub(crate) struct Graph {
    adj: HashMap<usize, Vec<usize>>,
    pred: HashMap<usize, Vec<usize>>,
    size: usize
}

impl Graph {
    /// Constructs a new graph from given adjacency and predecessor lists and the number of vertices.
    ///
    /// # Arguments
    ///
    /// * `adj` - A hashmap where each key is a vertex identifier and its value is a vector of vertices that can be reached from the key vertex.
    /// * `pred` - A hashmap similar to `adj` but each key's vector represents vertices that can reach the key vertex.
    /// * `size` - The number of vertices in the graph.
    pub(crate) fn new(adj: HashMap<usize, Vec<usize>>, pred: HashMap<usize, Vec<usize>>, size: usize) -> Self {
        Graph { adj, pred, size }
    }


    /// Resolves the adjacency relationships in the graph to simplify its structure based on transitive reduction.
    ///
    /// # Returns
    ///
    /// A string that represents the changes made to the graph, encoded in a special format.
    pub(crate) fn resolution_adjacence(&mut self) -> String {
        let mut lastmodified_line = 0;
        let mut prefixe = " 1 ".to_string();
        let mut suffixe = String::new();
        let mut cpt = 0;
        while cpt < self.size {
            if let Some(ligne) = self.adj.get(&cpt) {
                let mut sortante = 0;
                let mut index_sortante = 0;
                for (compte_valeur, &valeur) in ligne.iter().enumerate() {
                    if valeur == 1 {
                        index_sortante = compte_valeur;
                        sortante += 1;
                    }
                }

                if let Some(preds) = self.pred.get_mut(&cpt) {
                    // Top Case
                    if preds.is_empty() && sortante == 1 {
                        prefixe = format!("Int_0^x{}{}", index_sortante, prefixe);
                        suffixe += &format!("dx{}", cpt);
                        lastmodified_line = index_sortante;
                        self.adj.insert(cpt, vec![0; self.adj.len()]);
                        if let Some(pred_list) = self.pred.get_mut(&index_sortante) {
                            pred_list.retain(|&x| x != cpt);
                        }
                        cpt = 0;

                        continue;
                    }
                    // Bottom Case
                    else if preds.len() == 1 && sortante == 0 {
                        let pred_node = preds[0];
                        lastmodified_line = preds[0];
                        if let Some(pred_succs) = self.adj.get_mut(&pred_node) {
                            pred_succs[cpt] = 0; // Enlever le lien du prédécesseur vers ce nœud
                        }
                        self.pred.insert(cpt, vec![]);
                        prefixe = format!("Int_x{}^1{}", pred_node, prefixe);
                        suffixe += &format!("dx{}", cpt);
                        cpt = 0;
                        continue;
                    } else if preds.len() == 1 && sortante == 1 {
                        let var = preds[0];

                        // Accéder à l'élément de manière sécuritaire
                        if let Some(adj_list_var) = self.adj.get_mut(&var) {
                            if index_sortante < adj_list_var.len() {
                                adj_list_var[index_sortante] = 1;
                                adj_list_var[cpt] = 0;// Assurer que la connexion est établie
                            }
                        }

                        // Ajouter var aux prédécesseurs de index_sortante
                        self.pred.entry(index_sortante).or_insert_with(Vec::new).push(var);

                        // Supprimer cpt des prédécesseurs de index_sortante
                        if let Some(pred_list) = self.pred.get_mut(&index_sortante) {
                            pred_list.retain(|&x| x != cpt);  // Retirer cpt de la liste
                        }

                        // Remettre à zéro les connexions sortantes de cpt
                        self.adj.insert(cpt, vec![0; self.adj.len()]);
                        self.pred.insert(cpt, vec![]);  // Pas de prédécesseurs après la réinitialisation

                        // Mise à jour des préfixes et suffixes
                        prefixe = format!("Int_x{}^x{}{}", var, index_sortante, prefixe);
                        suffixe += &format!("dx{}", cpt);
                        lastmodified_line = cpt;
                        cpt = 0; // Réinitialiser la boucle
                        self.transitive_reduction();
                        continue;
                    }
                }
            }
            cpt += 1;
        }


        prefixe = "Int_0^1".to_owned() + &*prefixe;
        suffixe = suffixe + "dx" + &lastmodified_line.to_string();
        return prefixe + &suffixe;
    }

    /// Displays the adjacency matrix of the graph.

    fn afficher_matrice_adj(&self) {

        for i in 0..self.size {
            if let Some(row) = self.adj.get(&i) {
                let line: String = row.iter()
                    .map(|&num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", line);
            } else {
                println!("{}", "0 ".repeat(self.size).trim());
            }
        }
    }

    /// Performs a transitive reduction using a simplified Floyd-Warshall algorithm adapted for graphs represented with hash maps.
    /// We don't use it in the project, it doens't work for long paths
     fn transitive_reduction_floyd_warshall(&mut self) {
        // Use a temporary copy of the adjacency list for calculations without altering the original during iteration
        let adj_copy = self.adj.clone();

        for k in 0..self.size {
            if let Some(nodes_reachable_from_k) = adj_copy.get(&k) {
                for &i in nodes_reachable_from_k {
                    if let Some(succs_of_i) = self.adj.get_mut(&i) {
                        for &j in nodes_reachable_from_k {
                            if i != j && succs_of_i.contains(&k) && nodes_reachable_from_k.contains(&j) {
                                // If a path i -> k -> j exists, then the direct connection i -> j is redundant
                                succs_of_i.remove(j); // Remove the direct edge i -> j
                                if let Some(preds_of_j) = self.pred.get_mut(&j) {
                                    preds_of_j.retain(|&x| x != i); // Remove i from the predecessors of j
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Performs transitive reduction on the graph. This function first computes a topological order
    /// of the graph and then uses it to perform the transitive reduction.
    ///
    /// # Process
    /// 1. Compute the topological order of the graph.
    /// 2. Use the topological order to identify and remove redundant edges.
    /// 3. Rebuild the adjacency list based on the reduced graph.
    /// 4. Reconstruct the predecessor list using the updated adjacency list.
    fn transitive_reduction(&mut self) {
        let ordre_topologique = self.create_ordre_topologique();
        let ordre_reduit = transitive_reduction_topologique(&ordre_topologique);
        for (_, v) in self.adj.iter_mut() {
            *v = vec![0; self.size];
        }
        self.pred.clear();


        for (noeud, arrivee) in ordre_reduit {
            if let Some(adj_noeud) = self.adj.get_mut(&noeud) {
                // Modifier la valeur dans adj et predif arrivee < self.size {
                adj_noeud[arrivee] = 1;
            }
        }
        self.pred = construire_pred(&self.adj);
    }

    /// Generates a topological order for the graph by constructing an adjacency matrix and then processing it.
    ///
    /// # Returns
    /// A vector of tuples representing the topological order, where each tuple is a pair of vertices (source, target).
    ///
    /// # Example
    /// Given a graph with adjacency list representation, this method constructs an adjacency matrix
    /// and uses it to perform topological sorting.
    fn create_ordre_topologique(&mut self) -> Vec<(usize, usize)> {
        let mut matrice = Vec::new();

        let mut keys: Vec<usize> = self.adj.keys().copied().collect();
        keys.sort();

        for key in keys {
            if let Some(v) = self.adj.get(&key) {
                let row: Vec<i32> = v.iter().map(|&x| x as i32).collect(); // chat gpt m'a aidé
                //car j'avais un probleme de type vec<&vec<i32>> au lieu de vec<vec<i32>>
                matrice.push(row);
            }
        }

        create_ordre_topologique(matrice)
    }

}

mod tests {
    use crate::construire_adj;
    use crate::generator_matrix::{one_arity_matrix_generator, random_arity_matrix_generator};
    use super::*;

    #[test]
    fn test_resolution_hashmaps() {
        for i in 4..30 {
            for _ in 0..10 {
                println!("{}", i);
                let size = i;
                let matrice = random_arity_matrix_generator(size);
                let u = matrice.len();
                let adj = construire_adj(matrice);
                let pred = construire_pred(&adj);
                let mut g = Graph::new(adj, pred, u);

                g.resolution_adjacence();
                assert!(g.verify_all_zero(), "Couldn't resolve the DAG using BIT decomposition");
            }
        }
    }

    // Fonction helper pour vérifier que tous les nœuds sont résolus
    impl Graph {
        pub fn verify_all_zero(&self) -> bool {
            self.adj.values().all(|edges| edges.iter().all(|&x| x == 0))
        }
    }
}