use std::collections::HashSet;
use regex::Regex;
use crate::transitive_reduction::{apply_transitive_reduction, create_ordre_topologique, transitive_reduction_topologique};

/// Resolves adjacency matrices by applying transitive reduction and formatting output for integrals.
///
/// # Arguments
/// * `mut matrice` - A mutable reference to a vector of vector of i32s representing the adjacency matrix.
///
/// # Returns
/// A formatted string representing the integral expressions derived from the adjacency matrix after resolution.
pub(crate) fn resolution_adjacence(mut matrice: Vec<Vec<i32>>) -> String {
    let mut cpt = 0;
    let mut lastmodified_line = 0;
    let mut prefixe = " 1 ".to_string();
    let mut suffixe = String::new();
    while (cpt) < matrice.len() {
        let ligne = &matrice[cpt];
        let vne = verify_no_entry(matrice.clone(), cpt);
        let mut sortante = 0;
        let mut index_sortante = 0;
        for (compte_valeur, &valeur) in ligne.iter().enumerate() {
            if valeur == 1 {
                sortante += 1;
                index_sortante = compte_valeur;
            }
        }
        // Top Case
        if vne.0 == 0 && sortante == 1 {
            delete_vortex(&mut matrice, cpt);
            prefixe = ("Int_0^".to_owned() + "x" + &index_sortante.to_string() + &*prefixe);
            suffixe = suffixe + "dx" + &cpt.to_string();
            lastmodified_line = index_sortante;

            cpt = 0;
            continue;
        }
        //bottom case
        if vne.0 == 1 && sortante == 0 {
            //delete_edge(&mut matrice, cpt);
            matrice[vne.1][cpt] = 0;
            prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^1" + &*prefixe);
            suffixe = (suffixe + "dx" + &cpt.to_string());
            lastmodified_line = vne.1;
            cpt = 0;
            continue;
        }
        //Intermediate case
        if vne.0 == 1 && sortante == 1 {
            matrice[vne.1][index_sortante] = 1;
            matrice[vne.1][cpt] = 0;
            delete_vortex(&mut matrice, cpt);
            prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^x" + &index_sortante.to_string() + &*prefixe);
            suffixe = (suffixe + "dx" + &cpt.to_string());

            lastmodified_line = cpt;
            cpt = 0;
            let ordre_topo = create_ordre_topologique(matrice.clone());
            matrice = apply_transitive_reduction(matrice[0].len(), transitive_reduction_topologique(&ordre_topo));
            continue;
        }
        cpt += 1;
    }
    prefixe = "Int_0^1".to_owned() + &*prefixe;
    suffixe = suffixe + "dx" + &lastmodified_line.to_string();
    return prefixe.to_owned() + &*suffixe;
}


/// Verifies if there are no entries in a matrix row, used to assist in resolving adjacency matrices.
///
/// # Arguments
/// * `mut matrice` - A mutable reference to a vector of vector of i32s representing the adjacency matrix.
/// * `line_to_verify` - The line index to verify for entries.
///
/// # Returns
/// A tuple where the first element indicates the number of entries, and the second the line index.
fn verify_no_entry(mut matrice: Vec<Vec<i32>>, line_to_verify: usize) -> (i32, usize) {
    let mut res = 0;
    let mut res_ligne_entrante = 0;
    for (cpt_lignes, ligne) in matrice.iter().enumerate() {
        if cpt_lignes == line_to_verify {
            continue;
        } else {
            if ligne[line_to_verify] == 1 {
                res += 1;
                res_ligne_entrante = cpt_lignes;
            }
        }
    }
    (res, res_ligne_entrante)
}

/// Deletes a specific edge in an adjacency matrix, setting it to zero.
///
/// # Arguments
/// * `matrice` - A mutable reference to a vector of vector of i32s representing the adjacency matrix.
/// * `line_to_delete` - The line index where the edge should be deleted.
fn delete_edge(matrice: &mut Vec<Vec<i32>>, line_to_delete: usize) {
    let mut cpt = 0;
    for ligne in matrice {
        cpt += 1;
        if (ligne[line_to_delete] == 1) {
            ligne[line_to_delete] = 0;
        }
    }
}

/// Deletes all outgoing edges from a node in the matrix, effectively isolating it.
///
/// # Arguments
/// * `matrice` - A mutable reference to a vector of vector of i32s representing the adjacency matrix.
/// * `line_to_delete` - The line index of the node to isolate.
fn delete_vortex(matrice: &mut Vec<Vec<i32>>, line_to_delete: usize) {
    if line_to_delete < matrice.len() {
        matrice[line_to_delete] = vec![0; matrice[line_to_delete].len()];
    }
}


/// Applies a Floyd-Warshall based transitive reduction algorithm on a given matrix.
///
/// # Arguments
/// * `matrice` - A vector of vector of i32s representing the adjacency matrix to reduce.
///
/// # Returns
/// The transitive reduced matrix.
fn transitive_reduction_using_floyd_warshall(matrice: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

/// Verifies that a result string contains all expected node indices in a sequence from 0 to `taille-1`.
///
/// # Arguments
/// * `resultat` - The result string containing indices.
/// * `taille` - The expected number of indices.
///
/// # Returns
/// True if all indices are present and in order, otherwise false.
fn verifier_indices(resultat: &str, taille: usize) -> bool {
    let re = Regex::new(r"dx_(\d+)").unwrap();
    println!("{}", re);
    let mut indices = HashSet::new();

    for cap in re.captures_iter(resultat) {
        if let Some(matched) = cap.get(1) {
            if let Ok(num) = matched.as_str().parse::<usize>() {
                indices.insert(num);
            }
        }
    }

    (0..taille).all(|i| indices.contains(&i))
}




#[cfg(test)]
mod tests {
    fn resolution_adjacence_test(mut matrice: Vec<Vec<i32>>) -> (String, Vec<Vec<i32>>) {
        let mut cpt = 0;
        let mut lastmodified_line = 0;
        let mut prefixe = " 1 ".to_string();
        let mut suffixe = String::new();
        while (cpt) < matrice.len() {
            let ligne = &matrice[cpt];
            let vne = verify_no_entry(matrice.clone(), cpt);
            let mut sortante = 0;
            let mut index_sortante = 0;
            for (compte_valeur, &valeur) in ligne.iter().enumerate() {
                if valeur == 1 {
                    sortante += 1;
                    index_sortante = compte_valeur;
                }
            }
            // Top Case
            if vne.0 == 0 && sortante == 1 {
                delete_vortex(&mut matrice, cpt);
                prefixe = ("Int_0^".to_owned() + "x" + &index_sortante.to_string() + &*prefixe);
                suffixe = suffixe + "dx" + &cpt.to_string();
                lastmodified_line = index_sortante;

                cpt = 0;
                continue;
            }
            //bottom case
            if vne.0 == 1 && sortante == 0 {
                //delete_edge(&mut matrice, cpt);
                matrice[vne.1][cpt] = 0;
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^1" + &*prefixe);
                suffixe = (suffixe + "dx" + &cpt.to_string());
                lastmodified_line = vne.1;
                cpt = 0;
                continue;
            }
            //Intermediate case
            if vne.0 == 1 && sortante == 1 {
                matrice[vne.1][index_sortante] = 1;
                matrice[vne.1][cpt] = 0;
                delete_vortex(&mut matrice, cpt);
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^x" + &index_sortante.to_string() + &*prefixe);
                suffixe = (suffixe + "dx" + &cpt.to_string());

                lastmodified_line = cpt;
                cpt = 0;
                let ordre_topo = create_ordre_topologique(matrice.clone());
                matrice = apply_transitive_reduction(matrice[0].len(), transitive_reduction_topologique(&ordre_topo));
                continue;
            }
            cpt += 1;
        }
        prefixe = "Int_0^1".to_owned() + &*prefixe;
        suffixe = suffixe + "dx" + &lastmodified_line.to_string();
        (prefixe.to_owned() + &*suffixe, matrice)
    }
    use crate::generator_matrix::{add_node, generer_matrice_arite_hasard, generer_matrice_arite_un};
    use crate::verify_all_zero;
    use super::*;

    #[test]
    fn test_generer_matrice_arite_un() {
        for i in 4..30 {
            for _ in 0..10{
                println!("{}",i);
                let size = i; // Taille de la matrice pour le test
                let matrice = generer_matrice_arite_un(size);
                let (_, matrice_apres) = resolution_adjacence_test(matrice);
                assert!(verify_all_zero(matrice_apres), "Matrice non résolue");
            }
        }
    }


    #[test]
    fn test_generer_matrice_arite_hasard() {
        for i in 4..30 {
            for _ in 0..10{
                println!("{}",i);
                let size = i; // Taille de la matrice pour le test
                let matrice = generer_matrice_arite_hasard(size);
                let (_, matrice_apres) = resolution_adjacence_test(matrice);
                assert!(verify_all_zero(matrice_apres), "Matrice non résolue");
            }
        }
    }

    #[test]
    fn test_ajout_noeud_matrice() {
        for i in 4..30 {
            for _ in 0..10 {
                let mut matrice = generer_matrice_arite_un(i); // Generate une matrice initiale d'arité 1
                assert_eq!(matrice.len(), i, "Taille initiale incorrecte");
                add_node(&mut matrice); // Ajouter un nouveau nœud
                assert_eq!(matrice.len(), i+1, "Échec de l'ajout d'un noeud");
                assert!(matrice.iter().all(|row| row.len() == i+1), "Les lignes ne sont pas de la bonne longueur après ajout");
            }
        }
    }
}