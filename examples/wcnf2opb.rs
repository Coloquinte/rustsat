//! # wcnf2opb
//!
//! A small tool for converting DIMACS WCNF files to OPB.
//!
//! Useage: wcnf2opb [dimacs wcnf file] [opb output path]

use rustsat::instances::OptInstance;

macro_rules! print_usage {
    () => {{
        eprintln!("Useage: wcnf2opb [dimacs wcnf file] [opb output path]");
        panic!()
    }};
}

fn main() {
    let in_path = std::env::args().nth(1).unwrap_or_else(|| print_usage!());
    let out_path = std::env::args().nth(2).unwrap_or_else(|| print_usage!());

    let inst: OptInstance =
        OptInstance::from_dimacs_path(in_path).expect("error parsing the input file");

    inst.to_opb_path(out_path)
        .expect("io error writing the output file");
}