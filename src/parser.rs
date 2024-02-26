use pest::Parser;
use pest_derive::Parser;

use crate::integral::{Bound, IntegralSpec};

#[derive(Parser)]
#[grammar = "integral.pest"]
pub struct IntegralParser;

fn parse_bound(input: &str) -> Result<Bound, String> {
    //println!("[parse_bound ] input = {input}");
    match input {
        "0" => Ok(Bound::Zero),
        "1" => Ok(Bound::One),
        var => Ok(Bound::Var(var.to_string())),
    }
}

fn parse_bounds(mut bound_specs: pest::iterators::Pairs<Rule>) -> Result<(Bound, Bound), String> {
    let mut bound_spec = bound_specs.next();
    match parse_bound(bound_spec.unwrap().as_str()) {
        Err(e) => Err(e),
        Ok(b1) => {
            bound_spec = bound_specs.next();
            match parse_bound(bound_spec.unwrap().as_str()) {
                Err(e) => Err(e),
                Ok(b2) => Ok((b1, b2)),
            }
        }
    }
}

pub fn parse(source: &str) -> Result<IntegralSpec, String> {
    let res = IntegralParser::parse(Rule::input, source);

    if let Err(err) = res {
        return Err(err.to_string());
    }

    let mut int_spec = crate::integral::IntegralSpec::new();
    let mut int_bounds: Vec<(Bound, Bound)> = Vec::new();
    let mut int_vars: Vec<usize> = Vec::new();

    let mut int_parse = res.unwrap();
    let parts = int_parse
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .into_inner();

    for part in parts {
        //println!("part = {part}");
        match part.as_rule() {
            Rule::int_header => match parse_bounds(part.into_inner()) {
                Err(e) => return Err(e),
                Ok((b1, b2)) => int_bounds.push((b1, b2)),
            },
            Rule::int_footer => {
                let var_name = part.as_str()[1..].to_string();
                //println!("var = {var_name}");
                let var_ref = int_spec.register_var(var_name);
                int_vars.push(var_ref);
            }
            _ => panic!("Unexpected case"),

        }
    }

    //println!("Bounds = {:?}", int_bounds);
    //println!("Vars = {:?}", int_vars);

    if int_vars.len() != int_bounds.len() {
        return Err("Mismatch number of bounds vs. variables".to_string());
    }

    if int_vars.len() == 0 {
        return Ok(int_spec);
    }

    for ivar in 0..int_vars.len() {
        let var = int_vars[ivar];
        let ibound = int_bounds.len() - 1 - ivar;
        let (b1, b2) = &int_bounds[ibound];
        int_spec.push(var, b1.clone(), b2.clone());
    }

    return Ok(int_spec);
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_parse_comments() {
        let comment_ok = "// This is a test\n1";
        let res = parse(comment_ok);
        assert_eq!(res.unwrap().elements.len(), 0);

        let comment_ko = "-- This is not good\n\n1";
        let res = parse(comment_ko);
        assert!(res.is_err());
    }

    #[test]
    fn test_one_integral() {
        let spec = parse("1");
        assert_eq!(spec.unwrap().elements.len(), 0);
    }

    #[test]
    fn test_single_integral() {
        let spec = parse("Int_0^1 1 dx1");
        assert_eq!(spec.unwrap().elements.len(), 1);
        let spec = parse("Int_0^1 1 dx_1");
        assert_eq!(spec.unwrap().elements.len(), 1);
    }

    #[test]
    fn test_complex_integral() {
        let spec = parse("Int_0^1 Int_0^x3 Int_x3^1 Int_x3^1 Int_0^x3 1 dx1 dx5 dx4 dx2 dx3");
        assert_eq!(spec.unwrap().elements.len(), 5);

        let spec =
            parse("Int_0^1 Int_0^x_3 Int_x_3^1 Int_x_3^1 Int_0^x_3 1 dx_1 dx_5 dx_4 dx_2 dx_3");
        assert_eq!(spec.unwrap().elements.len(), 5);

        let spec_ko = parse("Int_0^1 Int_0^x2 Int_x2^1 Int_x2^1 Int_0^x2 1 dx4 dx3 dx1 dx2");
        assert!(spec_ko.is_err());
    }
}
