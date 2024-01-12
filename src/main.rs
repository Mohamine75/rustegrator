mod integral;
mod integrator;
mod maths;
mod parser;
mod version;

use clap::Parser;
use num::BigInt;
use std::process;

use integrator::integrate_spec;
use version::{VERSION_MAJOR, VERSION_MINOR};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in quiet mode (only show end result)
    #[arg(short, long)]
    quiet: bool,

    /// Compute linear extension count instead of integration result
    #[arg(short, long)]
    le: bool,

    /// Show formula at each integration step
    #[arg(short, long)]
    formula: bool,

    /// Show statistics at each integration step
    #[arg(short, long)]
    stats: bool,

    /// The integral to compute
    integral: String,
}

fn abort(header: &str, msg: &str) {
    eprintln!("{header}:\n  ==> {msg}");
    eprintln!("<Abort>");
    process::exit(1)
}

fn main() {
    let config = Args::parse();

    if !&config.quiet {
        println!("Rust(int)egrator v{VERSION_MAJOR}.{VERSION_MINOR}");
        println!("---------------------");
    }

    match parser::parse(&config.integral) {
        Err(e) => abort("Parse error", &e),
        Ok(spec) => {
            let nbvars = spec.var_map.len() + 1;
            match integrate_spec(&spec, config.quiet, config.formula, config.stats) {
                Err(e) => abort("Integration error", &e),
                Ok(res) => {
                    //println!("res = {:?}", res);
                    if config.le {
                        let nres =
                            res * maths::factorial(nbvars);
                        if !&config.quiet {
                            println!("#le = {nres}");
                        } else {
                            println!("{nres}");
                        }
                    } else { // keep rational form
                        let num = res.numer();
                        let den = BigInt::from(res.denom().clone());
                        println!("{num}/{den}");
                    }
                }
            }
        }
    }
}
