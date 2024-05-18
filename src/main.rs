mod integral;
mod integrator;
mod maths;
mod parser;
mod version;

mod generator_matrix;
mod transitive_reduction;

use clap::Parser;
use num::BigInt;
use std::process;


use integrator::integrate_spec;
use version::{VERSION_MAJOR, VERSION_MINOR};
use crate::integrator::{integrate_spec_file};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in quiet mode (only show end result)
    #[arg(short, long)]
    quiet: bool,

    //Debug mode
    #[arg(short, long)]
    debug: bool,
    /// Compute linear extension count instead of integration result
    #[arg(short, long)]
    le: bool,

    /// Take a file in
    #[arg(long)]
    file: Option<String>,
    /// Show formula at each integration step
    #[arg(short, long)]
    formula: bool,

    /// Show statistics at each integration step
    #[arg(short, long)]
    stats: bool,

    #[arg(long)]
    /// The integral to compute
    integral: Option<String>,
}

fn abort(header: &str, msg: &str) {
    eprintln!("{header}:\n  ==> {msg}");
    eprintln!("<Abort>");
    process::exit(1)
}
fn main() {
    let config = Args::parse();

    if !config.quiet {
        println!("Rust(int)egrator v{}.{}", VERSION_MAJOR, VERSION_MINOR);
        println!("---------------------");
    }

    // Parse the integral input to get a specification
    if let Some(integral) = config.integral {
        match parser::parse(&integral) {
            Err(e) => abort("Parse error", &e),
            Ok(spec) => {
                    let nbvars = spec.var_map.len() + 1;
                    match integrate_spec(
                        &spec,
                        config.quiet,
                        config.formula,
                        config.stats) {
                        Err(e) => abort("Integration error", &e),
                        Ok(res) => {
                            if config.le {
                                let nres = res * maths::factorial(nbvars);
                                if !config.quiet {
                                    println!("#le = {}", nres);
                                } else {
                                    println!("{}", nres);
                                }
                            } else {
                                let num = res.numer();
                                let den = BigInt::from(res.denom().clone());
                                println!("{}/{}", num, den);
                            }
                        }
                    }
                }
            }
        }

    else if let Some(file) = config.file {  // Access the inner `String` if it exists
        // Function to handle file processing should be called here
        integrate_spec_file(file);  // Pass the reference to the file string
    }else{ abort("MISSING ARG","You should run a file, select \
    \n --bin create_integrale in order to create integrals from integrales.txt
     \n --bin --<OPTIONS> rustegrator --<OPTIONAL_FILE> --<OPTIONAL_INTEGRAL>") }


}

