#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use zero_raf_core::{PublicRAFInputs, PrivateRAFInput};

pub fn main() {

    // Read in public inputs
    let _public_input: PublicRAFInputs = env::read();

    // Read in private inputs
    let _private_input: PrivateRAFInput = env::read();
    
    // Filter the private input diagnosis codes to only those that are mapped to HCCs

    // Apply Age & Sex edits 

    // Apply hierarchy to HCC list

    // Apply interactions to HCC list

    // Apply coefficients

    // Calculate RAF score by summing the values for each HCC

}
