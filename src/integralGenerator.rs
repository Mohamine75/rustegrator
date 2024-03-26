fn resolution_adjacence(mut matrice: Vec<Vec<i32>>, mut first_call: i32) -> String {
    let mut cpt = 0;
    let mut prefixe = String::new();
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
            if first_call == 0 {
                first_call += 1;
                prefixe = ("Int_0^".to_owned() + "x" + &index_sortante.to_string() + " 1 ");
                suffixe = ("dx".to_owned() + &cpt.to_string());
            } else {
                prefixe = ("Int_0^".to_owned() + "x" + &index_sortante.to_string() + " " + &*prefixe);
                suffixe = suffixe + " dx" + &cpt.to_string() + " ";
            }
            cpt = 0;
            continue;
        }
        //bottom case
        if vne.0 == 1 && sortante == 0 {
            let tmp = delete_edge(&mut matrice, cpt);
            if first_call == 0 {
                first_call += 1;
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^1 1 ");
                suffixe = ("dx".to_owned() + &cpt.to_string());
            } else {
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() + "^1 " + &*prefixe);
                suffixe = ( suffixe + " dx" + &cpt.to_string());
            }
            cpt = 0;
            continue;        }
        //Intermediate case
        if vne.0 == 1 && sortante == 1 {
            matrice[vne.1][cpt] = index_sortante as i32;
            delete_vortex(&mut matrice, cpt);
            if first_call == 0 {
                first_call += 1;
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() +"^"+&index_sortante.to_string()+" 1 ");
                suffixe = ("dx".to_owned() + &cpt.to_string());
            } else {
                prefixe = ("Int_".to_owned() + "x" + &vne.1.to_string() +"^"+&index_sortante.to_string() + " " + &*prefixe);
                suffixe = ( suffixe + " dx"  + &cpt.to_string());
            }
            cpt = 0;
            continue;
        }
        cpt+=1;
    }
    return prefixe.to_owned() + &*suffixe;
}

/*
    Function that verify if another vertex has an exit toward line_to_verify
 */
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

fn delete_edge(matrice: &mut Vec<Vec<i32>>, line_to_delete: usize) -> i32 {
    let mut cpt = 0;
    for ligne in matrice {
        cpt += 1;
        if (ligne[line_to_delete] == 1) {
            ligne[line_to_delete] = 0;
            return cpt;
        }
    }
    return 0;
}

fn delete_vortex(matrice: &mut Vec<Vec<i32>>, line_to_delete: usize) {
    if line_to_delete < matrice.len() {
        matrice[line_to_delete] = vec![0; matrice[line_to_delete].len()];
    }
}

fn main() {
    /*
    let ligne1 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne2 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
    let ligne3 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0];
    let ligne4 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne5 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
    let ligne6 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne7 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne8 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne9 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne10 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ligne11 = vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let ligne12 = vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let ligne13 = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0];
    let ligne14 = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];





    let matrice = vec![
        ligne1, ligne2, ligne3, ligne4, ligne5,
        ligne6, ligne7, ligne8, ligne9, ligne10,
        ligne11, ligne12, ligne13, ligne14,
    ];
    */


    /**
    Exemple du prof
    **/
    let ligne1 = vec![0, 0, 1, 0, 0, 0, 0, 0 ]; //a1
    let ligne2 = vec![0, 0, 1, 0, 0, 0, 0, 0 ]; //a2
    let ligne3 = vec![0, 0, 0, 1, 0, 0, 0, 1 ]; //x
    let ligne4 = vec![0, 0, 0, 0, 0, 0, 0, 0 ]; //b1
    let ligne5 = vec![0, 0, 0, 0, 0, 0, 0, 1 ]; // a3
    let ligne6 = vec![0, 0, 0, 0, 0, 0, 0, 0 ]; //b2
    let ligne7 = vec![0, 0, 0, 0, 0, 0, 0, 0 ]; //b3
    let ligne8 = vec![0, 0, 0, 0, 0, 1, 1, 0 ]; // y
    let matrice = vec![
        ligne1, ligne2, ligne3, ligne4, ligne5,
        ligne6, ligne7, ligne8
    ];
    let result = resolution_adjacence(matrice, 0);
    // println!("{}", matrice.len());
    println!("Result: {}", result);
}



/*
GARBAGE
 */


/*fn resolution_adjacence(matrice: Vec<Vec<i32>>, first_call: i32) -> String {
    let mut cpt = 0;
    for ligne in matrice {
        let mut vne = verify_no_entry(matrice,cpt);
        let mut sortante = 0;
        let mut index_sortante = 0;
        let mut compte_valeur = 0;
        for &valeur in ligne.iter() {
            if(valeur == 1) {
                sortante += 1;
                index_sortante = compte_valeur;
            }
            compte_valeur += 1;
        }
        // Top Case
        if vne.0== 0 && sortante == 1{
            if !matrice.is_empty() {
                matrice.remove(ligne);
            }
            if first_call == 0 {
                first_call +=1;
                return "Int_0^".to_owned() + ligne.to_string() + resolution_adjacence(matrice,first_call) + "1" + "dx" + ligne
            }
            return "Int_0^".to_owned() + ligne.to_string() + resolution_adjacence(matrice,first_call) + "dx" + ligne
        }
        //bottom case
        if vne.0 == 1 && sortante ==0 {
            let tmp = delete_edge(matrice,ligne);
            if first_call == 0 {
                first_call +=1;
                return "Int_^".to_owned() + tmp.to_string() + resolution_adjacence(matrice,first_call) + "1" + "dx" + ligne
            }
            return "Int_^".to_owned() + tmp.to_string() + resolution_adjacence(matrice,first_call) + "dx" + ligne

        }
        //Intermediate case
        if vne == 1 && sortante ==1 {
            matrice[vne.1][cpt] = index_sortante;
            matrice.remove(ligne);
            if first_call == 0 {
                first_call +=1;
                return "Int_^".to_owned() + vne.1.to_string() + resolution_adjacence(matrice,first_call) + "1" + "dx" + index_sortante
            }
            return "Int_^".to_owned() + vne.1.to_string() + resolution_adjacence(matrice,first_call) + "dx" + index_sortante
        }
        cpt += 1;
    }

    //No case could work, so can't do anything more with this matrice
    return ""
}
*/