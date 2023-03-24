use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Public data used in Guest to calculate RAF score for a patient
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicRAFInputs {
    // Coefficients published by CMS for each model HCC
    hcc_coefficients: HashMap<String, f32>,

    // Hierarchies of HCC superiority published by CMS
    hcc_hierarchies: HashMap<String, Vec<String>>,

    // Description labels for each HCC published by CMS
    hcc_labels: HashMap<String, String>,

    // Mapping of ICD-10 codes to HCCs published by CMS
    dx_to_cc: HashMap<String, Vec<String>>,
}

/// Private data used in Guest to calculate RAF score for a patient
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateRAFInput {
    // Array of diagnosis codes for a patient
    diagnosis_codes: Vec<String>,

    // Age of the patient
    age: i32,

    // Sex of the patient
    sex: String,

    // Eligibility code of the patient
    eligibility_code: String,

    // Entitlement reason code of the patient
    entitlement_reason_code: String,

    // Boolean indicating Medicaid status
    medicaid_status:    bool,

}
