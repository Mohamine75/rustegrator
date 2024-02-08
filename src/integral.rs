use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Bound {
    Zero,
    One,
    Var(String),
}

#[derive(Debug)]
pub struct IntegralSpec {
    pub var_map: HashMap<String, usize>,
    pub elements: Vec<(usize, Bound, Bound)>,
}

impl IntegralSpec {
    pub fn new() -> IntegralSpec {
        IntegralSpec {
            var_map: HashMap::new(),
            elements: vec![],
        }
    }

    pub fn register_var(&mut self, var_name: String) -> usize {
        // L'impression qu'on ajoute simplement une variable dans la hashMap à la fin
        let nb_vars = self.var_map.len();
        let var_ref = self.var_map.entry(var_name).or_insert(nb_vars);
        return *var_ref;
    }

    pub fn var_ref(&self, var_name: String) -> usize {
        match self.var_map.get(&var_name) {
            None => panic!("No such variable: {var_name} (please report)"),
            Some(i) => *i,
        }
    }

    pub fn var_name(&self, var_ref: usize) -> String {
        // Remark: slow function (use only for pretty-printing/debugging)
        for (vname, vref) in &self.var_map {
            if *vref == var_ref {
                return (*vname).clone();
            }
        }
        panic!("No such variable reference: {var_ref}");
    }

    pub fn push(&mut self, var_ref: usize, b1: Bound, b2: Bound) {
        self.elements.push((var_ref, b1, b2))
    }
}
