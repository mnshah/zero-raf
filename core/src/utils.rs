use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use csv::ReaderBuilder;

/*
    Finds the path to the CMS Data directory for the given performance year.
 */
pub fn get_cms_data_dir(performance_year: &str) -> String {

    let mut path = env::current_dir().unwrap();
    while !path.ends_with("zero-raf") {
        path.pop();
    }
    path.push("CMS-Data");
    path.push(performance_year);
    return path.to_str().unwrap().to_string();
}


/*
    Reads in label file and returns a dictionary of HCC to label
*/
pub fn read_hcc_labels(filename: &str) -> Result<HashMap<String, String>, csv::Error> {

    let mut labels = HashMap::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r#"\s*((?:HCC|CC)\d+)\s*=\s*"([^"]+)"#).unwrap();

    
    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(captures) = re.captures(&line) {
            let hcc = captures.get(1).unwrap().as_str();
            let label = captures.get(2).unwrap().as_str();
            labels.insert(hcc.to_string(), label.to_string());
        }
    }

    Ok(labels)
}

pub fn read_hier(filename: &str) -> Result<HashMap<String, Vec<String>>, csv::Error> {

    let mut hiers = HashMap::new();
    let pttr = Regex::new(r"%SET0\(CC=(\d+).+%STR\((.+)\)\)").unwrap();
    let file = File::open(filename)?;
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
    Ok(hiers)
}

/*
    Reads in a CSV file and returns a dictionary of HCC conditions to decimal coefficients
*/
pub fn read_hcc_coefficients(filename: &str) -> Result<HashMap<String, f32>, csv::Error> {

    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut headers = String::new();
    reader.read_line(&mut headers);
    let mut values = String::new();
    reader.read_line(&mut values);

    // Split headers into Vector of strings split by ","
    let headers: Vec<&str> = headers.split(",").collect();

    // Split values into Vector of strings split by ","
    let values: Vec<&str> = values.split(",").collect();

    // Assert headers and values are the same length
    assert_eq!(headers.len(), values.len());

    // Iterate through headers and values and insert into HashMap
    for i in 0..headers.len() {

        let mut key = headers[i].trim().to_string();
        key = key.replace("\"", "");

        map.insert(key, values[i].trim().parse::<f32>().unwrap());
    }
    Ok(map)
}

/*
    Reads in a CSV file and returns a dictionary of diagnosis codes to a list of 
    HCCs (hierarchical condition categories)
*/
pub fn read_dx_to_cc(filename: &str) -> Result<HashMap<String, Vec<String>>, csv::Error> {

    let file = File::open(filename)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(BufReader::new(file));
    let mut map = HashMap::<String, Vec<String>>::new();
    for result in reader.records() {
        let record = result?;
        let dx = &mut record[0].to_string();
        let cc = &mut record[1].to_string();
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

pub fn build_ne_reg_variable_list() -> Vec<String> {

    let mut ne_reg_variables = vec![];

    let le_65_age_segments = vec!["NEF0_34", "NEF35_44", "NEF45_54", "NEF55_59", "NEF60_64", "NEM0_34", "NEM35_44", "NEM45_54", "NEM55_59", "NEM60_64"];
    let ge_65_age_segments = vec!["NEF65", "NEF66", "NEF67", "NEF68", "NEF69", "NEF70_74", "NEF75_79", "NEF80_84", "NEF85_89", "NEF90_94", "NEF95_GT", 
                                             "NEM65", "NEM66", "NEM67", "NEM68", "NEM69", "NEM70_74", "NEM75_79", "NEM80_84", "NEM85_89", "NEM90_94", "NEM95_GT"];


    let left_permutations = vec!["NMCAID", "MCAID"];
    let right_permutations = vec!["ORIGDS", "NORIGDS"];

    for age_group in ge_65_age_segments {
        for left_perm in &left_permutations {
            for right_perm in &right_permutations {
                ne_reg_variables.push(format!("{}_{}_{}", left_perm, right_perm, age_group));
            }
        }
    }

    for age_group in le_65_age_segments {
        for left_perm in &left_permutations {
            ne_reg_variables.push(format!("{}_NORIGDS_{}", left_perm, age_group));
        }
    }

    return ne_reg_variables;

}

#[test]
fn can_locate_cms_data_dir() {
    let path = get_cms_data_dir("PY2023");
    assert!(path.ends_with("CMS-Data/PY2023"));

    let path = get_cms_data_dir("PY2022");
    assert!(path.ends_with("CMS-Data/PY2022"));

    let path = get_cms_data_dir("PY2021");
    assert!(path.ends_with("CMS-Data/PY2021"));
}

#[test]
fn can_build_hcc_labels_from_file() {
    let path = get_cms_data_dir("PY2023");
    let filename = path + "/V28115L3.txt";
    let labels = read_hcc_labels(&filename).unwrap();
    assert_eq!(labels.len(), 230);
    assert_eq!("HIV/AIDS ", labels.get("HCC1").unwrap());
    assert_eq!("Artificial Openings for Feeding or Elimination ", labels.get("HCC463").unwrap());
    assert_eq!("Seizure Disorders and Convulsions ", labels.get("CC201").unwrap());
    assert_eq!("Artificial Openings for Feeding or Elimination ", labels.get("CC463").unwrap());
}

#[test]
fn can_build_hcc_hierarchies_from_file() {
    let path = get_cms_data_dir("PY2023");
    let filename = path + "/V28115H1.TXT";
    let hiers = read_hier(&filename).unwrap();

    assert_eq!(hiers.len(), 60);

    let hcc154 = hiers.get("HCC154").unwrap();
    assert!(hcc154.contains(&"HCC155".to_string()));

    let hcc180 = hiers.get("HCC180").unwrap();
    assert!(hcc180.contains(&"HCC181".to_string()));
    assert!(hcc180.contains(&"HCC182".to_string()));
    assert!(hcc180.contains(&"HCC253".to_string()));
    assert!(hcc180.contains(&"HCC254".to_string()));

    let hcc17 = hiers.get("HCC17").unwrap();
    assert!(hcc17.contains(&"HCC18".to_string()));
    assert!(hcc17.contains(&"HCC19".to_string()));
    assert!(hcc17.contains(&"HCC20".to_string()));
    assert!(hcc17.contains(&"HCC21".to_string()));
    assert!(hcc17.contains(&"HCC22".to_string()));
    assert!(hcc17.contains(&"HCC23".to_string()));
    
    let hcc222 = hiers.get("HCC222").unwrap();
    assert!(hcc222.contains(&"HCC223".to_string()));
    assert!(hcc222.contains(&"HCC224".to_string()));
    assert!(hcc222.contains(&"HCC225".to_string()));
    assert!(hcc222.contains(&"HCC226".to_string()));
    assert!(hcc222.contains(&"HCC227".to_string()));
    
}

#[test]
fn can_buld_hcc_coefficients_from_file() {
    let path = get_cms_data_dir("PY2023");
    let filename = path + "/C2824T2N.csv";
    let coeffs = read_hcc_coefficients(&filename).unwrap();

    assert_eq!(coeffs.len(), 1237);
    assert_eq!(coeffs.get("CNA_F65_69").unwrap(), &0.33);
    assert_eq!(coeffs.get("CNA_HCC381").unwrap(), &1.075);
    assert_eq!(coeffs.get("INS_DIABETES_HF_V28").unwrap(), &0.209);
    assert_eq!(coeffs.get("SNPNE_NMCAID_ORIGDIS_NEM75_79").unwrap(), &2.039);
    assert_eq!(coeffs.get("SNPNE_MCAID_ORIGDIS_NEM95_GT").unwrap(), &2.573);
}

#[test]
fn can_build_dx_to_cc_from_file() {
    let path = get_cms_data_dir("PY2023");
    let filename = path + "/F2823T2N_FY22FY23.TXT";
    let dx2cc = read_dx_to_cc(&filename).unwrap();

    assert_eq!(dx2cc.keys().len(), 7770);

    let mut dx = dx2cc.get("B20").unwrap();
    assert!(dx.contains(&"HCC1".to_string()));

    dx = dx2cc.get("B4481").unwrap();
    assert!(dx.contains(&"HCC280".to_string()));
}

#[test]
fn can_build_ne_reg_variables() {
    let ne_reg_variables = build_ne_reg_variable_list();
    assert!(!ne_reg_variables.contains(&"MCAID_ORIGDS_NEF0_34".to_string()));
    assert_eq!(ne_reg_variables.len(), 108);
}