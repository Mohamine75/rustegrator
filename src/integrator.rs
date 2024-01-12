use std::collections::HashSet;
use std::{collections::HashMap, ops::Mul};
use num::{BigRational, Signed, BigInt};

use crate::integral::Bound;
use crate::integral::IntegralSpec;

// For now, we will use usize for degrees but maybe
// this should be generic using the num crate
#[derive(Debug)]
pub struct Poly {
    nbvars : usize,
    monos : HashMap<Vec<i64>, BigRational>
}

fn mk_one_mono(nbvars : usize) -> Vec<i64> {
    vec![0; nbvars]
}

fn mono_pp(spec : &IntegralSpec, mono : &Vec<i64>) -> String {
    let mut res = String::new();
    let mut first = true;
    for (var_ref, &d) in mono.iter().enumerate() {
        if d > 0 {
            if first {
                first = false;
            } else {
                res.push(' ');
            }
            res.push_str(&spec.var_name(var_ref));
            if d > 1 {
                res.push('^');
                res.push_str(&d.to_string());
            }
        }
    }
    if res.len() == 0 {
        return "1".to_string()
    } else {
        return res;
    }

}

fn antideriv_mono(mono : &Vec<i64>, var_num : usize) -> Vec<i64> {
    let mut res = Vec::new();
    for (i, &d) in mono.iter().enumerate() {
        if i == var_num {
            res.push(d + 1);
        } else {
            res.push(d);
        }
    }
    res
}

fn antideriv_coef(coef : &BigRational, mono : &Vec<i64>, var_num : usize) -> BigRational {
    return coef.mul(BigRational::new(BigInt::from(1), BigInt::from(1) + mono[var_num]));
}

fn mono_subst_var(mono : &Vec<i64>, subst_var : usize, by_var : usize) -> Vec<i64> {
     let mut nmono = mono.clone();
     nmono[by_var] += mono[subst_var];
     nmono[subst_var] = 0;
     nmono
}

fn mono_subst_const(mono : &Vec<i64>, subst_var : usize) -> Vec<i64> {
    let mut nmono = mono.clone();
    nmono[subst_var] = 0;
    nmono
}

impl Poly {
    pub fn new(nbvars : usize) -> Poly {
        let mut monos = HashMap::new();
        monos.insert(mk_one_mono(nbvars), BigRational::from_integer(BigInt::from(1)));
        Poly { nbvars, monos }
    }

    pub fn number_of_monos(&self) -> usize {
        self.monos.len()
    }

    pub fn number_of_distinct_coefs(&self) -> usize {
        let mut coefs = HashSet::new();
        for (_, coef) in self.monos.iter() {
            coefs.insert(coef);
        }
        coefs.len()    
    }

    pub fn is_constant(self) -> bool {
        if self.monos.len() != 1 { return false }
        for mono in self.monos {
            for vdeg in mono.0 {
                if vdeg > 0 { return false }
            }
        }
        return true;
    }

    pub fn as_constant(self) -> Option<BigRational> {
        if self.monos.len() != 1 { return None }
        for mono in self.monos {
            for vdeg in mono.0 {
                if vdeg > 0 { 
                    return None
                } else {
                    return Some(mono.1);
                }
            }
        }
        return None;
    }

    pub fn integrate(self, spec : &IntegralSpec, var : usize, from : &Bound, to : &Bound) -> Poly {
        let mut nmonos : HashMap<Vec<i64>, BigRational> = HashMap::new();
        //println!("[integrate] var={var}, from={:?}, to={:?}", from, to);

        for (mono, coef) in self.monos.iter() {
            let amono = antideriv_mono(mono, var);
            let acoef = antideriv_coef(coef, mono, var);
            let lmono : Option<Vec<i64>> = match &to {
                Bound::Zero => None,
                Bound::One => Some(mono_subst_const(&amono, var)),
                Bound::Var(by_var) => Some(mono_subst_var(&amono, var, spec.var_ref(by_var.clone()))) 
            };
            if lmono.is_some() {
                let llmono = lmono.unwrap();
                let entry = nmonos.entry(llmono).or_insert(BigRational::from_integer(BigInt::from(0)));
                *entry += &acoef;
            }

            let rmono : Option<Vec<i64>> = match &from {
                Bound::Zero => None,
                Bound::One => Some(mono_subst_const(&amono, var)),
                Bound::Var(by_var) => Some(mono_subst_var(&amono, var, spec.var_ref(by_var.clone())))
            };

            if rmono.is_some() {
                let rrmono = rmono.unwrap();
                let entry = nmonos.entry(rrmono).or_insert(BigRational::from_integer(BigInt::from(0)));
                *entry += -acoef;
            }

        }

        Poly { nbvars: self.nbvars, monos: nmonos }
    }
}

pub fn poly_pp(spec : &IntegralSpec, poly : &Poly) -> String {
    let mut res = String::new();
    let mut keys : Vec<&Vec<i64>> = poly.monos.keys().collect();
    keys.sort();
    keys.reverse();
    let mut first = true;
    for key in keys.into_iter() {
        let smono = mono_pp(spec, key);
        let coef = poly.monos.get(key).unwrap();
        if coef.is_positive() {
            if first {
                first = false;
            } else {
                res.push_str(" + ");
            }
            if *coef != BigRational::from_integer(BigInt::from(1)) || smono == "1" {
                res.push_str(&coef.to_string());
                if smono != "1" { res.push(' ') }
            }
        } else { // negative
            if first {
                first = false;
                res.push('-');
            } else {
              res.push_str(" - ");
            }
            if *coef != BigRational::from_integer(BigInt::from(-1)) || smono == "1" {
                res.push_str(&(-coef).to_string());
                if smono != "1" { res.push(' ') }
            }
        }
        if smono != "1" {
            res.push_str(&smono);
        }
    }
    res
}

pub fn integrate_spec(spec: &IntegralSpec, quiet_mode : bool, formula_mode : bool, stats_mode : bool) -> Result<BigRational, String> {
    let mut poly = Poly::new(spec.elements.len());
    let mut step = 1;

    for (var, from, to) in spec.elements.iter() {
        if !quiet_mode {
            println!("Step {step}:");
            if formula_mode {
                println!("  {:?}", poly_pp(spec, &poly));
            }
            if stats_mode {
                let nb_monos = &poly.number_of_monos();
                let nb_coefs = &poly.number_of_distinct_coefs();
                println!("  #monomials={nb_monos}   #coefficients={nb_coefs}");
            }
        }
        poly = poly.integrate(spec, *var, from, to);
        step += 1;
    }

    match poly.as_constant() {
        None => Err("Stuck integral".to_string()),
        Some(res) => Ok(res),
    }
}


#[cfg(test)]
mod tests {
    use num::{BigInt, BigRational};

    use crate::{integral::Bound, integral::IntegralSpec, integrator::poly_pp};

    use super::{Poly, mk_one_mono, mono_pp, antideriv_mono, antideriv_coef, mono_subst_var, mono_subst_const};

    #[test]
    fn test_one_monomial() {
        let one = mk_one_mono(8);
        assert_eq!(mono_pp(&IntegralSpec::new(), &one), "1");
    }

    #[test]
    fn test_one_antideriv() {
        let mut spec = IntegralSpec::new();
        let x1ref = spec.register_var("x1".to_string());
        let _x2ref = spec.register_var("x2".to_string());
        let x3ref = spec.register_var("x3".to_string());
        
        let one = mk_one_mono(3);
        let anti = antideriv_mono(&one, x1ref);
        assert_eq!(mono_pp(&spec, &anti), "x1");
        let anti2 = antideriv_mono(&anti, x1ref);
        assert_eq!(mono_pp(&spec, &anti2), "x1^2");
        let anti3 = antideriv_mono(&anti2, x3ref);
        assert_eq!(mono_pp(&spec, &anti3), "x1^2 x3");
    }

    #[test]
    fn test_coef_one_antideriv() {
        let mut spec = IntegralSpec::new();
        let x1ref = spec.register_var("x1".to_string());
        let one = mk_one_mono(8);
        let res = antideriv_coef(&BigRational::from_integer(BigInt::from(1)), &one, x1ref);
        assert_eq!(res.to_string(), "1");
    }

    #[test]
    fn test_mono_subst_var() {
        let mono = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let smono = mono_subst_var(&mono, 5, 0);
        assert_eq!(smono, vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    
    #[test]
    fn test_mono_subst_const() {
        let mono = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let smono = mono_subst_const(&mono, 5);
        assert_eq!(smono, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_integrate() {
        let mut spec = IntegralSpec::new();
        let x0ref = spec.register_var("x0".to_string());
        let x1ref = spec.register_var("x1".to_string());
        let x2ref = spec.register_var("x2".to_string());
        let x3ref = spec.register_var("x3".to_string());
        let x4ref = spec.register_var("x4".to_string());
        let x5ref = spec.register_var("x5".to_string());
        let x6ref = spec.register_var("x6".to_string());
        let x7ref = spec.register_var("x7".to_string());
        let x8ref = spec.register_var("x8".to_string());
        let x9ref = spec.register_var("x9".to_string());

        let p0 = Poly::new(10);
        assert_eq!(poly_pp(&spec, &p0), "1");
        let p1 = p0.integrate(&spec, x5ref, &Bound::Var("x0".to_string()), &Bound::One);
        assert_eq!(poly_pp(&spec, &p1), "-x0 + 1");
        let p2 = p1.integrate(&spec, x7ref, &Bound::Zero, &Bound::Var("x0".to_string()));
        assert_eq!(poly_pp(&spec, &p2), "-x0^2 + x0");
        let p3 = p2.integrate(&spec, x0ref, &Bound::Var("x3".to_string()), &Bound::One);
        assert_eq!(poly_pp(&spec, &p3), "1/3 x3^3 - 1/2 x3^2 + 1/6");
        let p4 = p3.integrate(&spec, x3ref, &Bound::Var("x1".to_string()), &Bound::One);
        assert_eq!(poly_pp(&spec, &p4), "-1/12 x1^4 + 1/6 x1^3 - 1/6 x1 + 1/12");
        let p5 = p4.integrate(&spec, x4ref, &Bound::Zero, &Bound::Var("x1".to_string()));
        assert_eq!(poly_pp(&spec, &p5), "-1/12 x1^5 + 1/6 x1^4 - 1/6 x1^2 + 1/12 x1");
        let p6 = p5.integrate(&spec, x8ref, &Bound::Var("x1".to_string()), &Bound::One);
        assert_eq!(poly_pp(&spec, &p6), "1/12 x1^6 - 1/4 x1^5 + 1/6 x1^4 + 1/6 x1^3 - 1/4 x1^2 + 1/12 x1");
        let p7 = p6.integrate(&spec, x1ref, &Bound::Var("x2".to_string()), &Bound::One);
        assert_eq!(poly_pp(&spec, &p7), "-1/84 x2^7 + 1/24 x2^6 - 1/30 x2^5 - 1/24 x2^4 + 1/12 x2^3 - 1/24 x2^2 + 1/280");
        let p8 = p7.integrate(&spec, x9ref, &Bound::Zero, &Bound::Var("x2".to_string()));
        assert_eq!(poly_pp(&spec, &p8), "-1/84 x2^8 + 1/24 x2^7 - 1/30 x2^6 - 1/24 x2^5 + 1/12 x2^4 - 1/24 x2^3 + 1/280 x2");
        let p9 = p8.integrate(&spec, x2ref, &Bound::Zero, &Bound::Var("x6".to_string()));
        assert_eq!(poly_pp(&spec, &p9), "-1/756 x6^9 + 1/192 x6^8 - 1/210 x6^7 - 1/144 x6^6 + 1/60 x6^5 - 1/96 x6^4 + 1/560 x6^2");
        let p10 = p9.integrate(&spec, x6ref, &Bound::Zero, &Bound::One);
        assert_eq!(poly_pp(&spec, &p10), "1/6720");
        // correct number of linear extensions: #le = 10! / 6720 = 540

    }

}