use serde::{Deserialize, Serialize};
use std::{collections::HashMap};

pub mod utils;

/// Public data used in Guest to calculate RAF score for a patient
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRAFInputs {
    // Coefficients published by CMS for each model HCC
    pub hcc_coefficients: HashMap<String, f32>,

    // Hierarchies of HCC superiority published by CMS
    pub hcc_hierarchies: HashMap<String, Vec<String>>,

    // Description labels for each HCC published by CMS
    pub hcc_labels: HashMap<String, String>,

    // Mapping of ICD-10 codes to HCCs published by CMS
    pub dx_to_cc: HashMap<String, Vec<String>>,

    // Normalization factor
    pub norm_factor: f32,
}

/// Private data used in Guest to calculate RAF score for a patient
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateRAFInput {
    // Array of diagnosis codes for a patient
    pub diagnosis_codes: Vec<String>,

    // Age of the patient
    pub age: i32,

    // Sex of the patient
    pub sex: String,

    // Eligibility code of the patient
    pub eligibility_code: String,

    // Entitlement reason code of the patient
    pub entitlement_reason_code: String,

    // Boolean indicating Medicaid status
    pub medicaid_status:    bool,

    // Boolean indicating whether the patient is institutionalized
    pub long_term_institutionalized: bool,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub raf_scores: HashMap<String, f32>,
    pub coefficients: HashMap<String, f32>,
}