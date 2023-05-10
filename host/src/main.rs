use zero_raf_core::{PrivateRAFInput};
use zero_raf_methods::{ZERO_RAF_ELF, ZERO_RAF_ID};
use risc0_zkvm::serde::{to_vec};
use risc0_zkvm::{Executor, ExecutorEnv, SessionReceipt};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // Make the prover.
    // let mut prover = Prover::new(ZERO_RAF_ELF, ZERO_RAF_ID).expect(
    //     "Prover should be constructed from valid method source code and corresponding method ID",
    // );

    // TODO: Implement communication with the guest here
    /*
        Phase 1: Setup the data to pass to the Guest code
        1. Read the diagnosis to condition categories from file
        2. Read the HCC coefficients from file
        3. Read the HCC labels from file
        4. Read the HCC hierachies from file
        5. Read the HCC short labels from file
    */

    /*
        Phase 2: Read in the demographic data for 1 or more patients to pass to the Guest code
    */
    let private_input = PrivateRAFInput {
        diagnosis_codes: vec!["A1234".to_string(), "B1234".to_string()],
        age: 70,
        sex: "M".to_string(),
        eligibility_code: "CNA".to_string(),
        entitlement_reason_code: "1".to_string(),
        medicaid_status: false,
    };

    println!("About to serialize private inputs");

    let receipt = raf(&private_input);
    receipt.verify(ZERO_RAF_ID).unwrap();

    // prover.add_input_u32_slice(&serde::to_vec(&private_input)?);

    /*
        Phase 3: Read in the diagnosis data for 1 or more patients to pass to the Guest code
    */

    /*
        Phase 4: Pass the data to the Guest code
    */

    /*
        Phase 5: Run the Guest code to output the RAF for each patient
    */
    // let receipt = prover.run()
    //     .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(ZERO_RAF_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    println!("Success! Saved the receipt to ");

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here
    Ok(())
}

fn raf(private_inputs: &PrivateRAFInput) -> SessionReceipt {

    // let mut prover =
    //     Prover::new(ZERO_RAF_ELF).expect("Prover should be constructed from valid ELF binary");

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(private_inputs).unwrap())
        .build();

    // Make the Executor.
    let mut exec = Executor::from_elf(env, ZERO_RAF_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();


    // // Prove the session to produce a receipt.
    return session.prove().unwrap();
    // prover.add_input_u32_slice(&to_vec(public_inputs).unwrap());
    // prover.add_input_u32_slice(&to_vec(private_inputs).unwrap());
    // let receipt = prover.run().unwrap();
    // return receipt;

}


#[test]
fn can_send_to_prover() {

    // let private_input = PrivateRAFInput {
    //     diagnosis_codes: vec!["A1234".to_string(), "B1234".to_string()],
    //     age: 70,
    //     sex: "M".to_string(),
    //     eligibility_code: "CNA".to_string(),
    //     entitlement_reason_code: "1".to_string(),
    //     medicaid_status: false,
    // };

    // println!("Private Input: {}", private_input.diagnosis_codes[0]);

    // let env = ExecutorEnv::builder()
    // .add_input(&to_vec(&private_input).unwrap())
    // .build();

    // println!("About to make executor");

    // // Make the Executor.
    // let mut exec = Executor::from_elf(env, ZERO_RAF_ELF).unwrap();

    // println!("About to run executor");

    // // Run the executor to produce a session.
    // let session = exec.run().unwrap();

    // // Prove the session to produce a receipt.
    // session.prove().unwrap();

    // return;
}

#[test] 
fn can_generate_hcc_labels() {}

#[test] 
fn can_generate_hcc_hierarchies() {}

#[test] 
fn can_generate_hcc_coefficients() {}

#[test] 
fn can_generate_dx_to_cc() {}

#[test] 
fn can_serialize_public_inputs() {}

#[test]
fn can_serialize_private_input() {

    let private_input = PrivateRAFInput {
        diagnosis_codes: vec!["A1234".to_string(), "B1234".to_string()],
        age: 70,
        sex: "M".to_string(),
        eligibility_code: "CNA".to_string(),
        entitlement_reason_code: "1".to_string(),
        medicaid_status: false,
    };

    println!("About to serialize private inputs");
    let _input_data = &to_vec(&private_input).unwrap();

}