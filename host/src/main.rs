use methods::{ZERO_RAF_ELF, ZERO_RAF_ID};
use risc0_zkvm::Prover;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use csv::ReaderBuilder;
use risc0_zkvm::serde::to_vec;

use zero_raf_core::{PublicRAFInputs, PrivateRAFInput};


/*
    Reads in a CSV file and returns a dictionary of HCC conditions to decimal coefficients
*/
fn read_hcc_coefficients(filename: &str) -> Result<HashMap<String, f32>, csv::Error> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(file));
    let mut map = HashMap::new();
    for result in reader.records() {
        let record = result?;
        if let Some((key, value)) = record.iter().next().zip(record.iter().skip(1).next()) {
            let coeff: f32 = value.parse().unwrap();
            map.insert(key.to_string(), coeff);
        }
    }
    Ok(map)
}

/*
    Reads in a CSV file and returns a dictionary of diagnosis codes to a list of condition categories
*/
fn read_dx_to_cc(filename: &str) -> Result<HashMap<String, Vec<String>>, csv::Error> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(file));
    let mut map = HashMap::<String, Vec<String>>::new();
    for result in reader.records() {
        let mut dx = &result?[0].to_string();
        let mut cc = &result?[1].to_string();
        // Append HCC to condition category to match the format in the HCC coefficients file 
        cc.insert_str(0, "HCC");

        // If the diagnosis code is already in the dictionary, append the new condition category to the existing value
        if map.contains_key(dx) {
            map.get_mut(dx).unwrap().push(cc.to_string());
        } else {
            map.insert(dx.to_string(), vec![cc.to_string()]);
        }
    }
    Ok(map)
}

/*
    Reads in label file and returns a dictorionary of HCC to label
*/
fn read_hcc_labels(filename: &str) -> Result<HashMap<String, String>, csv::Error> {
    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(BufReader::new(file));
    let mut map = HashMap::new();
    for result in reader.records() {
        let record = result?;
        if let Some((key, value)) = record.iter().next().zip(record.iter().skip(1).next()) {
            map.insert(key.to_string(), value.to_string());
        }
    }
    Ok(map)
}

fn get_agesex(age: i32, sex: &str) -> (&str, &str) {
    let mut agegroup = "Adult";
    let mut agesexvar = "MAGE_LAST_21_24";

    if age < 1 {
        agesexvar = match sex {
            "M" => "AGE0_MALE",
            _ => &(sex.to_owned() + "AGE_LAST_0_0"),
        };
        agegroup = "Infant";
    } else if age < 2 {
        agesexvar = match sex {
            "M" => "AGE1_MALE",
            _ => &(sex.to_owned() + "AGE_LAST_1_1"),
        };
        agegroup = "Infant";
    } else if age < 5 {
        agegroup = "Child";
        agesexvar = &(sex.to_owned() + "AGE_LAST_2_4");
    } else if age < 10 {
        agegroup = "Child";
        agesexvar = &(sex.to_owned() + "AGE_LAST_5_9");
    } else if age < 15 {
        agegroup = "Child";
        agesexvar = &(sex.to_owned() + "AGE_LAST_10_14");
    } else if age < 21 {
        agegroup = "Child";
        agesexvar = &(sex.to_owned() + "AGE_LAST_15_20");
    } else if age < 25 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_21_24");
    } else if age < 30 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_25_29");
    } else if age < 35 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_30_34");
    } else if age < 40 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_35_39");
    } else if age < 45 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_40_44");
    } else if age < 50 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_45_49");
    } else if age < 55 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_50_54");
    } else if age < 60 {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_55_59");
    } else {
        agegroup = "Adult";
        agesexvar = &(sex.to_owned() + "AGE_LAST_60_GT");
    }

    return (agesexvar, agegroup);
}


fn read_hier(fn_name: &str) -> HashMap<String, Vec<String>> {
    let mut hiers = HashMap::new();
    let pttr = Regex::new(r"%SET0\(CC=(\d+).+%STR\((.+)\)\)").unwrap();
    let file = File::open(fn_name).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let matches = pttr.captures(&line);
        if let Some(caps) = matches {
            let k = "HCC".to_owned() + &caps[1];
            let v: Vec<String> = caps[2]
                .split(',')
                .map(|x| "HCC".to_owned() + x.trim())
                .collect();
            hiers.insert(k, v);
        }
    }
    hiers
}

fn main() {
    // Make the prover.
    let mut prover = Prover::new(ZERO_RAF_ELF, ZERO_RAF_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    // TODO: Implement communication with the guest here
    /*
        Phase 1: Setup the data to pass to the Guest code
        1. Read the diagnosis to condition categories from file
        2. Read the HCC coefficients from file
        3. Read the HCC labels from file
        4. Read the HCC hierachies from file
        5. Read the HCC short labels from file
    */

    let mut hcc_labels = match read_hcc_labels("hcc_labels.txt") {
        Ok(map) => map,
        Err(err) => HashMap::new(),
    };

    let mut hcc_hiers = read_hier("hcc_hier.txt");

    let mut hcc_coeffs = match read_hcc_coefficients("hcc_coeff.txt") {
        Ok(map) => map,
        Err(err) => HashMap::new(),
    };
    let mut dx_to_cc = match read_dx_to_cc("hcc_diag.txt") {
        Ok(map) => map,
        Err(err) => HashMap::new(),
    };

    let public_inputs = PublicRAFInputs {
        hcc_coefficients: hcc_coeffs,
        hcc_hierarchies: hcc_hiers,
        hcc_labels: hcc_labels,
        dx_to_cc: dx_to_cc,
    };
    prover.add_input_u32_slice(&risc0_zkvm::serde::to_vec(&public_inputs).unwrap());

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

    prover.add_input_u32_slice(&risc0_zkvm::serde::to_vec(&private_input).unwrap());



    /*
        Phase 3: Read in the diagnosis data for 1 or more patients to pass to the Guest code
    */

    /*
        Phase 4: Pass the data to the Guest code
    */

    /*
        Phase 5: Run the Guest code to output the RAF for each patient
    */
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(ZERO_RAF_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    println!("Success! Saved the receipt to ");

    // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here

    Ok(());
}
