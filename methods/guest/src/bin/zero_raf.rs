#![no_main]
// #![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use zero_raf_core::{PublicRAFInputs, PrivateRAFInput};
use std::collections::HashMap;
use std::sync::Once;

// Define a Struct to capture the original RAF coefficient and whether the attribute is 
// true for the patient. The struct will be used by a global HashMap to determine if the 
// coefficient should be applied to the RAF score.
#[derive(Debug)]
pub struct RAFAttribute {
    pub coefficient: f32,
    pub is_true: bool,
}

// Create a global HashMap to store the RAF attributes. The HashMap will be initialized
// in the main function using the public HCC Coefficients and `is_true` set to false.
// As the Guest code continues, applicable RAF attributes will be set to true.
static mut GLOBAL_RAF_MAP: Option<HashMap<String, RAFAttribute>> = None;
static INIT: Once = Once::new();

fn _get_global_raf_map() -> &'static mut HashMap<String, RAFAttribute> {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_RAF_MAP = Some(HashMap::new());
        });
        GLOBAL_RAF_MAP.as_mut().unwrap()
    }
}

// 1  MACRO NAME:  V28I0ED1
//                 UDXG update V0123 for V28 model (payment HCCs only). 
//                 ICD10 codes valid in FY20 through FY23.
// 2  PURPOSE:     age/sex edits on ICD10: some edits are mandatory, 
//                 others - are based on MCE list to check
//                 if age or sex for a beneficiary is within the
//                 range of acceptable age/sex, if not- CC is set to 
//                 -1.0 - invalid
// 3  PARAMETERS:  AGE   - beneficiary age variable calculated by DOB
//                         from a person level file
//                 SEX   - beneficiary SEX variable in a person level file
//                 ICD10  - diagnosis variable in a diagnosis file

// 4  COMMENTS:    1. Age format AGEFMT0 and sex format SEXFMT0 are 
//                    parameters in the main macro. They have to 
//                    correspond to the years of data

//                 2. If ICD10 code does not have any restriction on age
//                    or sex then the corresponding format puts it in "-1"

//                 3. AGEL format sets lower limits for age
//                    AGEU format sets upper limit for age
//                    for specific edit categories:
//                      "0"= "0 newborn (age 0)      "
//                      "1"= "1 pediatric (age 0 -17)"
//                      "2"= "2 maternity (age 9 -64)"
//                      "3"= "3 adult (age 15+)      "

//                 4. SEDITS - parameter for the main macro
// *Translated from V28I0ED1.TXT for more details*
fn icd_10_edits() {

}

// This function defines the Age & Sex grouping for the person with the given age, sex,
// and original reason enrolled in Medicare. 
// *Translated from AGESEXV2.TXT for more details*
fn _age_sex_v2(age: i32, sex: &str, orec: &str) -> HashMap<String, bool> {
    
    // Define a map with keys associated with the different groupings based on AGE, SEX, and OREC
    let mut age_sex_map = HashMap::<String, bool>::new();
    // Enrollee keys: F0_34 F35_44 F45_54 F55_59 F60_64 F65_69 
    //                F70_74 F75_79 F80_84 F85_89 F90_94 F95_GT
    //                M0_34  M35_44 M45_54 M55_59 M60_64 M65_69
    //                M70_74 M75_79 M80_84 M85_89 M90_94 M95_GT
    let enrollee_keys = vec![
        "F0_34", "F35_44", "F45_54", "F55_59", "F60_64", "F65_69", "F70_74", "F75_79", "F80_84", "F85_89", "F90_94", "F95_GT",
        "M0_34", "M35_44", "M45_54", "M55_59", "M60_64", "M65_69", "M70_74", "M75_79", "M80_84", "M85_89", "M90_94", "M95_GT"
    ];
    for key in enrollee_keys { age_sex_map.insert(key.to_string(), false); }
    let mut cat_key = String::from("F");
    if sex == "M" { cat_key = String::from("M"); }
    if age <= 34 {
        cat_key.push_str("0_34");
    } 
    else if age >= 35 && age < 45 {
        cat_key = cat_key + "35_44";
    }
    else if age >= 45 && age < 55 {
        cat_key = cat_key + "45_54";
    }
    else if age >= 55 && age < 60 {
        cat_key = cat_key + "55_59";
    }
    else if age >= 60 && age < 65 {
        cat_key = cat_key + "60_64";
    }
    else if age >= 65 && age < 70 {
        cat_key = cat_key + "65_69";
    }
    else if age >= 70 && age < 75 {
        cat_key = cat_key + "70_74";
    }
    else if age >= 75 && age < 80 {
        cat_key = cat_key + "75_79";
    }
    else if age >= 80 && age < 85 {
        cat_key = cat_key + "80_84";
    }
    else if age >= 85 && age < 90 {
        cat_key = cat_key + "85_89";
    }
    else if age >= 90 && age < 95 {
        cat_key = cat_key + "90_94";
    }
    else if age >= 95 {
        cat_key = cat_key + "95_GT";
    }

    let val = age_sex_map.get_mut(&cat_key).unwrap();
    *val = true;
    
    // New Enrollee keys: NEF0_34  NEF35_44 NEF45_54 NEF55_59 NEF60_64
    //                    NEF65    NEF66    NEF67    NEF68    NEF69
    //                    NEF70_74 NEF75_79 NEF80_84 NEF85_89 NEF90_94
    //                    NEF95_GT
    //                    NEM0_34  NEM35_44 NEM45_54 NEM55_59 NEM60_64
    //                    NEM65    NEM66    NEM67    NEM68    NEM69
    //                    NEM70_74 NEM75_79 NEM80_84 NEM85_89 NEM90_94
    //                    NEM95_GT
    let new_enrollee_keys = vec![
        "NEF0_34", "NEF35_44", "NEF45_54", "NEF55_59", "NEF60_64", "NEF65", "NEF66", "NEF67", "NEF68", "NEF69", "NEF70_74", "NEF75_79", "NEF80_84", "NEF85_89", "NEF90_94", "NEF95_GT",
        "NEM0_34", "NEM35_44", "NEM45_54", "NEM55_59", "NEM60_64", "NEM65", "NEM66", "NEM67", "NEM68", "NEM69", "NEM70_74", "NEM75_79", "NEM80_84", "NEM85_89", "NEM90_94", "NEM95_GT"
    ];
    for key in new_enrollee_keys { age_sex_map.insert(key.to_string(), false); }
    let mut ne_cat_key = String::from("NEF");
    if sex == "M" { ne_cat_key = String::from("NEM"); }
    if age <= 34 {
        ne_cat_key = ne_cat_key + "0_34";
    } 
    else if age >= 35 && age < 45 {
        ne_cat_key = ne_cat_key + "35_44";
    }
    else if age >= 45 && age < 55 {
        ne_cat_key = ne_cat_key + "45_54";
    }
    else if age >= 55 && age < 60 {
        ne_cat_key = ne_cat_key + "55_59";
    }
    else if age >= 60 && age < 65 {
        ne_cat_key = ne_cat_key + "60_64";
    }
    // if age == 64 and orec is not 0 
    else if age == 64 && orec != "0" {
        ne_cat_key = ne_cat_key + "60_64";
    }
    else if age == 64 && orec == "1" {
        ne_cat_key = ne_cat_key + "65";
    }
    else if age == 65 {
        ne_cat_key = ne_cat_key + "65";
    }
    else if age == 66 {
        ne_cat_key = ne_cat_key + "66";
    }
    else if age == 67 {
        ne_cat_key = ne_cat_key + "67";
    }
    else if age == 68 {
        ne_cat_key = ne_cat_key + "68";
    }
    else if age == 69 {
        ne_cat_key = ne_cat_key + "69";
    }
    else if age >= 70 && age < 75 {
        ne_cat_key = ne_cat_key + "70_74";
    }
    else if age >= 75 && age < 80 {
        ne_cat_key = ne_cat_key + "75_79";
    }
    else if age >= 80 && age < 85 {
        ne_cat_key = ne_cat_key + "80_84";
    }
    else if age >= 85 && age < 90 {
        ne_cat_key = ne_cat_key + "85_89";
    }
    else if age >= 90 && age < 95 {
        ne_cat_key = ne_cat_key + "90_94";
    }
    else if age >= 95 {
        ne_cat_key = ne_cat_key + "95_GT";
    }

    let ne_val = age_sex_map.get_mut(&ne_cat_key).unwrap();
    *ne_val = true;

    //Other keys ORIGDS  - originally disabled dummy variable
    //           DISABL  - disabled dummy variable
    // DISABL = (&AGEF < 65 & &OREC ne "0");
    // %* originally disabled;
    // ORIGDS  = (&OREC = '1')*(DISABL = 0);

    let disabl = age < 65 && orec != "0";
    age_sex_map.insert(String::from("DISABL"), disabl);
    age_sex_map.insert(String::from("ORIGDS"), orec == "1" && !disabl);

    return age_sex_map;
}


// 1  MACRO NAME: V28115H1
// 2  PURPOSE:    HCC HIERARCHIES: version 28 of HCCs,
//                only payment model HCCs are included
// 3  COMMENT:    it is assumed that:
//                -number of payment model HCCs are placed into global macro 
//                 variable N_CC in the main program
//                -the following arrays are set in the main program 
//                 ARRAY C(&N_CC)   &CClist
//                 ARRAY HCC(&N_CC) &HCClist
//                -format ICD to CC creates only &N_CC CMS CCs
fn _apply_hierarchy(model_hcc_hiers: HashMap<String, Vec<String>>, patient_hcc_list: &Vec<String>) -> Vec<String> {
    // Create an initial index with the patient HCC list values mapped to true

    let mut superior_hccs: Vec<String> = vec![];
    let mut inferior_hccs: HashMap<&String,bool> = HashMap::new();


    for hcc in patient_hcc_list.iter() {
        if model_hcc_hiers.contains_key(hcc) & !inferior_hccs.contains_key(hcc) {

            superior_hccs.push(hcc.clone());
            for inferior_hcc in model_hcc_hiers.get(hcc).unwrap() {
                inferior_hccs.entry(&inferior_hcc).or_insert(true);
            }
        }
    }

    return superior_hccs;

}




/* 
Diagnostic categories:
    CANCER_V28          = MAX(HCC17, HCC18, HCC19, HCC20, HCC21, HCC22, HCC23);
    DIABETES_V28        = MAX(HCC35, HCC36, HCC37, HCC38);
    CARD_RESP_FAIL      = MAX(HCC211, HCC212, HCC213);

    HF_V28              = MAX(HCC221, HCC222, HCC223, HCC224, HCC225, HCC226);
    CHR_LUNG_V28        = MAX(HCC276, HCC277, HCC278, HCC279, HCC280);
    KIDNEY_V28          = MAX(HCC326, HCC327, HCC328, HCC329);
    SEPSIS              = HCC2;
    gSubUseDisorder_V28 = MAX(HCC135, HCC136, HCC137, HCC138, HCC139);
    gPsychiatric_V28    = MAX(HCC151, HCC152, HCC153, HCC154, HCC155);   
    NEURO_V28           = MAX(HCC180, HCC181, HCC182, HCC190, HCC191, HCC192, HCC195, HCC196, HCC198, HCC199);
    ULCER_V28           = MAX(HCC379, HCC380, HCC381, HCC382);   

Community models interactions:
    DIABETES_HF_V28               = DIABETES_V28*HF_V28;
    HF_CHR_LUNG_V28               = HF_V28*CHR_LUNG_V28;
    HF_KIDNEY_V28                 = HF_V28*KIDNEY_V28;
    CHR_LUNG_CARD_RESP_FAIL_V28   = CHR_LUNG_V28*CARD_RESP_FAIL;
    HF_HCC238_V28                 = HF_V28*HCC238;
    gSubUseDisorder_gPsych_V28    = gSubUseDisorder_V28*gPsychiatric_V28;

Institutional model:
    DISABLED_CANCER_V28          = DISABL*CANCER_V28;
    DISABLED_NEURO_V28           = DISABL*NEURO_V28;
    DISABLED_HF_V28              = DISABL*HF_V28;
    DISABLED_CHR_LUNG_V28        = DISABL*CHR_LUNG_V28;
    DISABLED_ULCER_V28           = DISABL*ULCER_V28;
*/

static KEYS_FOR_NUM_PAYMENT_HCCS : [&'static str; 10] = ["D0", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9"];

fn _apply_interactions(patient_hcc_list : &Vec<String>, is_disabled : bool) -> Vec<String> {

    // Create a Counter object to capture all the interactions.
    let mut interaction_counter = HashMap::<&str, &u32>::new();

    // Iterate through patient_hcc_list and add to the interaction_counter
    for hcc in patient_hcc_list {
        interaction_counter.entry(&hcc).or_insert(&1);
    }

    // Create a Counter object to hold the diagnostic categories
    let mut diagnostic_counter = HashMap::<&str, &u32>::new();

    match is_disabled {
        true => {
            diagnostic_counter.insert(&"DISABL", &1);
        }
        false => {
            diagnostic_counter.insert(&"DISABL", &0);
        }
    }

    let cancer_vals = vec![interaction_counter["HCC17"], 
                           interaction_counter["HCC18"], 
                           interaction_counter["HCC19"], 
                           interaction_counter["HCC20"], 
                           interaction_counter["HCC21"], 
                           interaction_counter["HCC22"], 
                           interaction_counter["HCC23"]];
    diagnostic_counter.insert(&"CANCER_V28", cancer_vals.iter().max().unwrap());

    let diabetes_vals = vec![interaction_counter["HCC35"], 
                             interaction_counter["HCC36"], 
                             interaction_counter["HCC37"], 
                             interaction_counter["HCC38"]];
    diagnostic_counter.insert(&"DIABETES_V28", diabetes_vals.iter().max().unwrap());

    let card_resp_fail_vals = vec![interaction_counter["HCC211"], 
                                   interaction_counter["HCC212"], 
                                   interaction_counter["HCC213"]];
    diagnostic_counter.insert(&"CARD_RESP_FAIL", card_resp_fail_vals.iter().max().unwrap());

    let hf_vals = vec![interaction_counter["HCC221"], 
                       interaction_counter["HCC222"], 
                       interaction_counter["HCC223"], 
                       interaction_counter["HCC224"], 
                       interaction_counter["HCC225"], 
                       interaction_counter["HCC226"]];
    diagnostic_counter.insert(&"HF_V28", hf_vals.iter().max().unwrap());

    let chr_lung_vals = vec![interaction_counter["HCC276"], 
                             interaction_counter["HCC277"], 
                             interaction_counter["HCC278"], 
                             interaction_counter["HCC279"], 
                             interaction_counter["HCC280"]];
    diagnostic_counter.insert(&"CHR_LUNG_V28", chr_lung_vals.iter().max().unwrap());

    let kidney_vals = vec![interaction_counter["HCC326"], 
                           interaction_counter["HCC327"], 
                           interaction_counter["HCC328"], 
                           interaction_counter["HCC329"]];
    diagnostic_counter.insert(&"KIDNEY_V28", kidney_vals.iter().max().unwrap());

    let sepsis_vals = vec![interaction_counter["HCC2"]];
    diagnostic_counter.insert(&"SEPSIS", sepsis_vals.iter().max().unwrap());

    let sub_use_disorder_vals = vec![interaction_counter["HCC135"], 
                                    interaction_counter["HCC136"], 
                                    interaction_counter["HCC137"], 
                                    interaction_counter["HCC138"], 
                                    interaction_counter["HCC139"]];
    diagnostic_counter.insert(&"gSubUseDisorder_V28", sub_use_disorder_vals.iter().max().unwrap());

    let psychiatric_vals = vec![interaction_counter["HCC151"], 
                                interaction_counter["HCC152"], 
                                interaction_counter["HCC153"], 
                                interaction_counter["HCC154"], 
                                interaction_counter["HCC155"]];
    diagnostic_counter.insert(&"gPsychiatric_V28", psychiatric_vals.iter().max().unwrap());

    let neuro_vals = vec![interaction_counter["HCC180"], 
                          interaction_counter["HCC181"], 
                          interaction_counter["HCC182"], 
                          interaction_counter["HCC190"], 
                          interaction_counter["HCC191"], 
                          interaction_counter["HCC192"], 
                          interaction_counter["HCC195"], 
                          interaction_counter["HCC196"], 
                          interaction_counter["HCC198"], 
                          interaction_counter["HCC199"]];
    diagnostic_counter.insert(&"NEURO_V28", neuro_vals.iter().max().unwrap());

    let ulcer_vals = vec![interaction_counter["HCC379"], 
                          interaction_counter["HCC380"], 
                          interaction_counter["HCC381"], 
                          interaction_counter["HCC382"]]; 
    diagnostic_counter.insert(&"ULCER_V28", ulcer_vals.iter().max().unwrap());

    // Create the community model interactions
    let diabetes_hf_v28 = &(diagnostic_counter["DIABETES_V28"] * diagnostic_counter["HF_V28"]);
    interaction_counter.entry(&"DIABETES_HF_V28").or_insert(diabetes_hf_v28);

    let hf_chr_lung_v28 = &(diagnostic_counter["HF_V28"] * diagnostic_counter["CHR_LUNG_V28"]);
    interaction_counter.entry(&"HF_CHR_LUNG_V28").or_insert(hf_chr_lung_v28);

    let hf_kidney_v28 = &(diagnostic_counter["HF_V28"] * diagnostic_counter["KIDNEY_V28"]);
    interaction_counter.entry(&"HF_KIDNEY_V28").or_insert(hf_kidney_v28);

    let chr_lung_card_resp_fail_v28 = &(diagnostic_counter["CHR_LUNG_V28"] * diagnostic_counter["CARD_RESP_FAIL"]);
    interaction_counter.entry(&"CHR_LUNG_CARD_RESP_FAIL_V28").or_insert(chr_lung_card_resp_fail_v28);

    let hcc238_val = &(diagnostic_counter["HF_V28"] * interaction_counter["HCC238"]);
    interaction_counter.entry(&"HF_HCC238_V28").or_insert(hcc238_val);
    
    let gsub_use_disorder_gpsych_v28 = &(diagnostic_counter["gSubUseDisorder_V28"] * diagnostic_counter["gPsychiatric_V28"]);
    interaction_counter.entry(&"gSubUseDisorder_gPsych_V28").or_insert(&gsub_use_disorder_gpsych_v28);

    // Create the institutional model interactions
    /*
    DISABLED_CANCER_V28          = DISABL*CANCER_V28;
    DISABLED_NEURO_V28           = DISABL*NEURO_V28;
    DISABLED_HF_V28              = DISABL*HF_V28;
    DISABLED_CHR_LUNG_V28        = DISABL*CHR_LUNG_V28;
    DISABLED_ULCER_V28           = DISABL*ULCER_V28;
    */
    let disabl_val = diagnostic_counter["DISABL"];
    let disabl_cancer = &(disabl_val * diagnostic_counter["CANCER_V28"]);
    interaction_counter.entry(&"DISABLED_CANCER_V28").or_insert(disabl_cancer);
    
    let disabl_neuro = &(disabl_val * diagnostic_counter["NEURO_V28"]);
    interaction_counter.entry(&"DISABLED_NEURO_V28").or_insert(disabl_neuro);

    let disabl_hf = &(disabl_val * diagnostic_counter["HF_V28"]);
    interaction_counter.entry(&"DISABLED_HF_V28").or_insert(disabl_hf);

    let disabl_chr_lung = &(disabl_val * diagnostic_counter["CHR_LUNG_V28"]);
    interaction_counter.entry(&"DISABLED_CHR_LUNG_V28").or_insert(disabl_chr_lung);

    let disabl_ulcer = &(disabl_val * diagnostic_counter["ULCER_V28"]);
    interaction_counter.entry(&"DISABLED_ULCER_V28").or_insert(disabl_ulcer);


    // Add keys based on number of HCCs
    if patient_hcc_list.len() >= 10 {
        interaction_counter.insert(&"D10P", &1);
    } else if patient_hcc_list.len() >= 1 {
        let key = KEYS_FOR_NUM_PAYMENT_HCCS[patient_hcc_list.len()];
        interaction_counter.entry(key).or_insert(&1);

    }

    let final_interactions: Vec<String> = interaction_counter.keys().map(|x| x.to_string()).collect();
    return final_interactions;
}

pub fn main() {

    // Read in public inputs
    let _public_input: PublicRAFInputs = env::read();

    // Read in private inputs
    let _private_input: PrivateRAFInput = env::read();

    // Iterate through hcc_coefficients and initialize the GLOBAL_RAF_MAP
    let mut global_raf_map = _get_global_raf_map();
    for ele in _public_input.hcc_coefficients.iter() {
        let label = String::from(ele.0);
        global_raf_map.entry(label).or_insert(RAFAttribute {
            coefficient: *ele.1,
            is_true: false,
        });
    }
    
    // Filter the private input diagnosis codes to only those that are mapped to HCCs
    let mut hcc_list = vec![];
    for dx in _private_input.diagnosis_codes {
        if _public_input.dx_to_cc.contains_key(&dx) {
            hcc_list.push(_public_input.dx_to_cc.get(&dx).unwrap().clone());
        }
    }
    let flattened_hcc_list = hcc_list.into_iter().flatten().collect::<Vec<String>>();


    // Apply Age & Sex edits 
    let age_sex_map = _age_sex_v2(_private_input.age, &_private_input.sex, &_private_input.entitlement_reason_code);

    // TODO: Apply ICD-10 edits (MCE data should be an input parameter)

    // Apply hierarchy to HCC list
    let final_hcc_list = _apply_hierarchy(_public_input.hcc_hierarchies, &flattened_hcc_list);

    // Apply interactions to HCC list
    let final_interactions = _apply_interactions(&final_hcc_list, _private_input.entitlement_reason_code == "0");

    // Apply coefficients

    // Calculate RAF score by summing the values for each HCC

}
