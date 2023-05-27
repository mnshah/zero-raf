use risc0_zkvm::guest::env;
use risc0_zkvm::guest::env::log;
risc0_zkvm::guest::entry!(main);
use zero_raf_core::utils::{build_ne_reg_variable_list};
use zero_raf_core::{PublicRAFInputs, PrivateRAFInput, Journal};
use std::collections::HashMap;
use std::sync::Once;



// Define a Struct to capture the original RAF coefficient and whether the attribute is 
// true for the patient. The struct will be used by a global HashMap to determine if the 
// coefficient should be applied to the RAF score.
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
// fn icd_10_edits() {

// }

// This function defines the Age & Sex grouping for the person with the given age, sex,
// and original reason enrolled in Medicare. 
// *Translated from AGESEXV2.TXT for more details*
fn _age_sex_v2(age: i32, sex: &str, orec: &str) -> Vec<String> {
    
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
    for key in &enrollee_keys { age_sex_map.insert(key.to_string(), false); }
    let mut key_index = enrollee_keys.len();
    if age <= 34 {
        key_index = 0;
    } 
    else if age >= 35 && age < 45 {
        key_index = 1;
    }
    else if age >= 45 && age < 55 {
        key_index = 2;
    }
    else if age >= 55 && age < 60 {
        key_index = 3;
    }
    else if age >= 60 && age < 65 {
        key_index = 4;
    }
    else if age >= 65 && age < 70 {
        key_index = 5;
    }
    else if age >= 70 && age < 75 {
        key_index = 6;
    }
    else if age >= 75 && age < 80 {
        key_index = 7;
    }
    else if age >= 80 && age < 85 {
        key_index = 8;
    }
    else if age >= 85 && age < 90 {
        key_index = 9;
    }
    else if age >= 90 && age < 95 {
        key_index = 10;
    }
    else if age >= 95 {
        key_index = 11;
    }
    if sex == "M" { key_index += 12; }

    let cat_key = enrollee_keys[key_index];
    let val = age_sex_map.get_mut(cat_key).unwrap();
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
    for key in &new_enrollee_keys { age_sex_map.insert(key.to_string(), false); }
    let mut key_index = new_enrollee_keys.len();

    if age <= 34 {
        key_index = 0;
    } 
    else if age >= 35 && age < 45 {
        key_index = 1;
    }
    else if age >= 45 && age < 55 {
        key_index = 2;
    }
    else if age >= 55 && age < 60 {
        key_index = 3;
    }
    else if age >= 60 && age < 65 {
        key_index = 4;
    }
    // if age == 64 and orec is not 0 
    if age == 64 && orec != "0" {
        key_index = 4;
    }
    else if age == 64 && orec == "0" {
        key_index = 5;
    }
    else if age == 65 {
        key_index = 5;   
    }
    else if age == 66 {
        key_index = 6;
    }
    else if age == 67 {
        key_index = 7
    }
    else if age == 68 {
        key_index = 8;
    }
    else if age == 69 {
        key_index = 9;
    }
    else if age >= 70 && age < 75 {
        key_index = 10;
    }
    else if age >= 75 && age < 80 {
        key_index = 11;
    }
    else if age >= 80 && age < 85 {
        key_index = 12;
    }
    else if age >= 85 && age < 90 {
        key_index = 13;
    }
    else if age >= 90 && age < 95 {
        key_index = 14;
    }
    else if age >= 95 {
        key_index = 15;
    }

    if sex == "M" { key_index += 16; }
    let ne_cat_key = new_enrollee_keys[key_index];
    let ne_val = age_sex_map.get_mut(ne_cat_key).unwrap();
    *ne_val = true;

    //Other keys ORIGDS  - originally disabled dummy variable
    //           DISABL  - disabled dummy variable
    // DISABL = (&AGEF < 65 & &OREC ne "0");
    // %* originally disabled;
    // ORIGDS  = (&OREC = '1')*(DISABL = 0);

    let disabl = age < 65 && orec != "0";
    age_sex_map.insert(String::from("DISABL"), disabl);
    age_sex_map.insert(String::from("ORIGDS"), orec == "1" && !disabl);

    age_sex_map.retain(|_, v| *v == true);

    return age_sex_map.keys().map(|s| s.to_string()).collect();

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
fn _apply_hierarchy(model_hcc_hiers: &HashMap<String, Vec<String>>, patient_hcc_list: &Vec<String>) -> Vec<String> {
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

    let mut cancer_vals = vec![0];
    if interaction_counter.contains_key(&"HCC17") || 
       interaction_counter.contains_key(&"HCC18") ||
       interaction_counter.contains_key(&"HCC19") ||
       interaction_counter.contains_key(&"HCC20") ||
       interaction_counter.contains_key(&"HCC21") ||
       interaction_counter.contains_key(&"HCC22") ||
       interaction_counter.contains_key(&"HCC23") {
        cancer_vals.push(1);
    }
    diagnostic_counter.insert(&"CANCER_V28", cancer_vals.iter().max().unwrap());

    let mut diabetes_vals = vec![0];
    if interaction_counter.contains_key(&"HCC35") ||
       interaction_counter.contains_key(&"HCC36") ||
       interaction_counter.contains_key(&"HCC37") ||
       interaction_counter.contains_key(&"HCC38") { 
        diabetes_vals.push(1);
    }   
    diagnostic_counter.insert(&"DIABETES_V28", diabetes_vals.iter().max().unwrap());

    let mut card_resp_fail_vals = vec![0];
    if interaction_counter.contains_key("HCC211") ||
       interaction_counter.contains_key("HCC212") ||
       interaction_counter.contains_key("HCC213") {
        card_resp_fail_vals.push(1);
       }    
    diagnostic_counter.insert(&"CARD_RESP_FAIL", card_resp_fail_vals.iter().max().unwrap());

    let mut hf_vals = vec![0];
    if interaction_counter.contains_key("HCC221") ||
       interaction_counter.contains_key("HCC222") ||
       interaction_counter.contains_key("HCC223") ||
       interaction_counter.contains_key("HCC224") || 
       interaction_counter.contains_key("HCC225") ||
       interaction_counter.contains_key("HCC226") {
        hf_vals.push(1);
    }    
    diagnostic_counter.insert(&"HF_V28", hf_vals.iter().max().unwrap());

    let mut chr_lung_vals = vec![0];
    if interaction_counter.contains_key("HCC276") ||
       interaction_counter.contains_key("HCC277") ||
       interaction_counter.contains_key("HCC278") ||
       interaction_counter.contains_key("HCC279") ||
       interaction_counter.contains_key("HCC280") {
        chr_lung_vals.push(1);
    }
    diagnostic_counter.insert(&"CHR_LUNG_V28", chr_lung_vals.iter().max().unwrap());

    let mut kidney_vals = vec![0];
    if interaction_counter.contains_key("HCC326") ||
       interaction_counter.contains_key("HCC327") ||
       interaction_counter.contains_key("HCC328") ||
       interaction_counter.contains_key("HCC329") {
        kidney_vals.push(1);
    }        
    diagnostic_counter.insert(&"KIDNEY_V28", kidney_vals.iter().max().unwrap());

    let mut sepsis_vals = vec![0];
    if interaction_counter.contains_key("HCC2") { 
        sepsis_vals.push(1);
    }
    diagnostic_counter.insert(&"SEPSIS", sepsis_vals.iter().max().unwrap());

    let mut sub_use_disorder_vals = vec![0];
    if interaction_counter.contains_key("HCC135") ||
       interaction_counter.contains_key("HCC136") ||
       interaction_counter.contains_key("HCC137") ||
       interaction_counter.contains_key("HCC138") ||
       interaction_counter.contains_key("HCC139") {
        sub_use_disorder_vals.push(1);
    }
    diagnostic_counter.insert(&"gSubUseDisorder_V28", sub_use_disorder_vals.iter().max().unwrap());

    let mut psychiatric_vals = vec![0];
    if interaction_counter.contains_key("HCC151") ||
       interaction_counter.contains_key("HCC152") ||
       interaction_counter.contains_key("HCC153") ||
       interaction_counter.contains_key("HCC154") ||
       interaction_counter.contains_key("HCC155") {
        psychiatric_vals.push(1);
    }
    diagnostic_counter.insert(&"gPsychiatric_V28", psychiatric_vals.iter().max().unwrap());

    let mut neuro_vals = vec![0];
    if interaction_counter.contains_key("HCC180") ||
       interaction_counter.contains_key("HCC181") ||
       interaction_counter.contains_key("HCC182") ||
       interaction_counter.contains_key("HCC190") ||
       interaction_counter.contains_key("HCC191") ||
       interaction_counter.contains_key("HCC192") ||
       interaction_counter.contains_key("HCC195") ||
       interaction_counter.contains_key("HCC196") ||
       interaction_counter.contains_key("HCC198") ||
       interaction_counter.contains_key("HCC199") {
        neuro_vals.push(1);
    } 
    diagnostic_counter.insert(&"NEURO_V28", neuro_vals.iter().max().unwrap());

    let mut ulcer_vals = vec![0];
    if interaction_counter.contains_key("HCC379") ||
       interaction_counter.contains_key("HCC380") ||
       interaction_counter.contains_key("HCC381") ||
       interaction_counter.contains_key("HCC382") {
        ulcer_vals.push(1);
    } 
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

    let mut hf_hcc238_v28 = 0;
    if interaction_counter.contains_key("HCC238") {
        hf_hcc238_v28 = diagnostic_counter["HF_V28"] * diagnostic_counter["HCC238"];
    } 
    interaction_counter.entry(&"HF_HCC238_V28").or_insert(&hf_hcc238_v28);
    
    
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
    interaction_counter.retain(|&_k, v| *v > &0);

    let final_interactions: Vec<String> = interaction_counter.keys().map(|x| x.to_string()).collect();

    return final_interactions;
}


/*

 %LET ADDZ=%STR(                                                                                
  D1 D2 D3 D4 D5 D6 D7 D8 D9 D10P                                                                 
  );     

 %*list of HCCs included in models;
 %LET HCCV28_list115 = %STR(      
HCC1 HCC2 HCC6 HCC17 HCC18 HCC19 HCC20 HCC21 HCC22 HCC23 HCC35 HCC36
HCC37 HCC38 HCC48 HCC49 HCC50 HCC51 HCC62 HCC63 HCC64 HCC65 HCC68 HCC77
HCC78 HCC79 HCC80 HCC81 HCC92 HCC93 HCC94 HCC107 HCC108 HCC109 HCC111
HCC112 HCC114 HCC115 HCC125 HCC126 HCC127 HCC135 HCC136 HCC137 HCC138
HCC139 HCC151 HCC152 HCC153 HCC154 HCC155 HCC180 HCC181 HCC182 HCC190
HCC191 HCC192 HCC193 HCC195 HCC196 HCC197 HCC198 HCC199 HCC200 HCC201
HCC202 HCC211 HCC212 HCC213 HCC221 HCC222 HCC223 HCC224 HCC225 HCC226
HCC227 HCC228 HCC229 HCC238 HCC248 HCC249 HCC253 HCC254 HCC263 HCC264
HCC267 HCC276 HCC277 HCC278 HCC279 HCC280 HCC282 HCC283 HCC298 HCC300
HCC326 HCC327 HCC328 HCC329 HCC379 HCC380 HCC381 HCC382 HCC383 HCC385
HCC387 HCC397 HCC398 HCC399 HCC401 HCC402 HCC405 HCC409 HCC454 HCC463
     );
 %LET HCClist=&HCCV28_list115;
 %LET CClist=&CCV28_list115;

 %* age/sex variables for Community Aged regression;
 %LET COMM_REGA= %STR(&AGESEXVA
                      &orig_int
                      &HCClist
                      &INTERRACC_VARSA 
                      &ADDZ);

 %LET AGESEXVA=                                    F65_69
                F70_74 F75_79 F80_84 F85_89 F90_94 F95_GT
                                                   M65_69
                M70_74 M75_79 M80_84 M85_89 M90_94 M95_GT;

 %*interaction variables for Community Aged regressions;                                             
 %LET INTERRACC_VARSA=%STR(DIABETES_HF_V28                                                                      
                           HF_CHR_LUNG_V28                                                                      
                           HF_KIDNEY_V28                                                                        
                           CHR_LUNG_CARD_RESP_FAIL_V28                                                          
                           HF_HCC238_V28                                                                         
           );                                                                                        
                                                                                                     
 %LET ORIG_INT =%STR(OriginallyDisabled_Female OriginallyDisabled_Male);  
 OriginallyDisabled_Female= ORIGDS*(SEX='2');
 OriginallyDisabled_Male  = ORIGDS*(SEX='1');

 %LET COMM_REGD= %STR(&AGESEXVD
                      &HCClist
                      &INTERRACC_VARSD
                      &ADDZ);


 %* age/sex variables for Community Disabled regression;
 %LET AGESEXVD= F0_34  F35_44 F45_54 F55_59 F60_64
                M0_34  M35_44 M45_54 M55_59 M60_64;

 %*interaction variables for Community Disabled regressions;                                         
 %LET INTERRACC_VARSD=%STR(DIABETES_HF_V28
                           HF_CHR_LUNG_V28
                           HF_KIDNEY_V28
                           CHR_LUNG_CARD_RESP_FAIL_V28
                           HF_HCC238_V28
                           gSubUseDisorder_gPsych_V28
           );

    %&SCOREMAC(PVAR=SCORE_COMMUNITY_NA,  RLIST=&COMM_REGA, CPREF=CNA_);
    %&SCOREMAC(PVAR=SCORE_COMMUNITY_ND,  RLIST=&COMM_REGD, CPREF=CND_);
    %&SCOREMAC(PVAR=SCORE_COMMUNITY_FBA, RLIST=&COMM_REGA, CPREF=CFA_);
    %&SCOREMAC(PVAR=SCORE_COMMUNITY_FBD, RLIST=&COMM_REGD, CPREF=CFD_);
    %&SCOREMAC(PVAR=SCORE_COMMUNITY_PBA, RLIST=&COMM_REGA, CPREF=CPA_);
    %&SCOREMAC(PVAR=SCORE_COMMUNITY_PBD, RLIST=&COMM_REGD, CPREF=CPD_);
*/

static COMM_REGA: [&str; 30] = ["F65_69", "F70_74", "F75_79", "F80_84", "F85_89", "F90_94", "F95_GT",
                                "M65_69", "M70_74", "M75_79", "M80_84", "M85_89", "M90_94", "M95_GT",
                                "DIABETES_HF_V28", "HF_CHR_LUNG_V28", "HF_KIDNEY_V28", 
                                "CHR_LUNG_CARD_RESP_FAIL_V28", "HF_HCC238_V28", "gSubUseDisorder_gPsych_V28",
                                "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10P"];

fn _get_community_model_reg_a_score(model: String, private_input: &PrivateRAFInput, public_inputs: &PublicRAFInputs, all_raf_attributes: &Vec<String>) -> f32 {

    let global_map = _get_global_raf_map();
    let mut comm_reg_a_score = 0.0;
    
    // Gather all the coefficient values for variable that apply to Community Aged Model A
    // 1. Age Sex Variables:  F65_69, F70_74, F75_79, F80_84, F85_89, F90_94, F95_GT
    //                        M65_69, M70_74, M75_79, M80_84, M85_89, M90_94, M95_GT
    // 2. ORIG_INT:  OriginallyDisabled_Female, OriginallyDisabled_Male
    // 3. HCCList: &HCCV28_list115;
    // 4. Interaction Vars:  DIABETES_HF_V28, HF_CHR_LUNG_V28, HF_KIDNEY_V28, CHR_LUNG_CARD_RESP_FAIL_V28
    //                       HF_HCC238_V28, gSubUseDisorder_gPsych_V28
    // 5. Payment Variables:  D1 D2 D3 D4 D5 D6 D7 D8 D9 D10P                                                                                
   
    let mut raf_keys = all_raf_attributes
                                                    .iter()
                                                    .filter(|x| COMM_REGA.contains(&x.as_str()) || 
                                                                          public_inputs.hcc_labels.contains_key(*x))
                                                    .map(|x| format!("{}_{}", model, x))
                                                    .collect::<Vec<String>>();

    if all_raf_attributes.contains(&String::from("ORIGDS")) {
        let mut origds_key = format!("{}_OriginallyDisabled_Female", model);
        if private_input.sex == "M" {
            origds_key = format!("{}_OriginallyDisabled_Male", model);
        }
        raf_keys.push(origds_key);
    }
                                            
    raf_keys.iter_mut().for_each(|x| {
        if global_map.contains_key(x) {
            comm_reg_a_score += global_map.get(x).unwrap().coefficient;
            global_map.get_mut(x).unwrap().is_true = true;
        }
    });
    
    return comm_reg_a_score;
}

static COMM_REGD: [&str; 26] = ["F0_34", "F35_44", "F45_54", "F55_59", "F60_64",
                                "M0_34", "M35_44", "M45_54", "M55_59", "M60_64",
                                "DIABETES_HF_V28", "HF_CHR_LUNG_V28", "HF_KIDNEY_V28", 
                                "CHR_LUNG_CARD_RESP_FAIL_V28", "HF_HCC238_V28", "gSubUseDisorder_gPsych_V28",
                                "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10P"];

fn _get_community_model_reg_d_score(model: String, public_inputs: &PublicRAFInputs, all_raf_attributes: &Vec<String>) -> f32 {

    let global_map = _get_global_raf_map();
    let mut comm_reg_d_score = 0.0;
    
    // Gather all the coefficient values for variable that apply to Community Disabled Model D
    // 1. Age Sex Variable:  F0_34, F35_44, F45_54, F55_59 F60_64
    //                       M0_34  M35_44 M45_54 M55_59 M60_64
    // 2. HCCList: &HCCV28_list115;
    // 3. Interaction Vars: DIABETES_HF_V28, HF_CHR_LUNG_V28, HF_KIDNEY_V28, CHR_LUNG_CARD_RESP_FAIL_V28
    //                      HF_HCC238_V28, gSubUseDisorder_gPsych_V28
    // 4. Payment Variables:  D1 D2 D3 D4 D5 D6 D7 D8 D9 D10P                                                                            
   
    let mut raf_keys = all_raf_attributes
                                                    .iter()
                                                    .filter(|x| COMM_REGD.contains(&x.as_str()) || 
                                                                          public_inputs.hcc_labels.contains_key(*x))
                                                    .map(|x| format!("{}_{}", model, x))
                                                    .collect::<Vec<String>>();

    raf_keys.iter_mut().for_each(|x| {
        if global_map.contains_key(x) {
            comm_reg_d_score += global_map.get(x).unwrap().coefficient;
            global_map.get_mut(x).unwrap().is_true = true;
        }
    });
   
    return comm_reg_d_score;


} 

//  %*variables for Institutional regression;
// %LET INST_REG = %STR(&AGESEXV
//    LTIMCAID  ORIGDS
//    &HCClist
//    &INTERRACI_VARS
//    &ADDZ);

static INST_REG: [&str; 45] = ["F0_34", "F35_44", "F45_54", "F55_59", "F60_64", "F65_69", "F70_74", "F75_79", "F80_84", "F85_89", "F90_94", "F95_GT",
                               "M0_34", "M35_44", "M45_54", "M55_59", "M60_64", "M65_69", "M70_74", "M75_79", "M80_84", "M85_89", "M90_94", "M95_GT",
                               "LTIMCAID", "ORIGDS", "DIABETES_HF_V28", "HF_CHR_LUNG_V28", "HF_KIDNEY_V28", "CHR_LUNG_CARD_RESP_FAIL_V28", "DISABLED_CANCER_V28",
                               "DISABLED_NEURO_V28", "DISABLED_HF_V28", "DISABLED_CHR_LUNG_V28", "DISABLED_ULCER_V28",
                               "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D10P"];

fn _get_institutional_reg_score(model: String, public_inputs: &PublicRAFInputs, all_raf_attributes: &Vec<String>) -> f32 {
    let global_map = _get_global_raf_map();
    let mut inst_reg_score = 0.0;

    let mut raf_keys = all_raf_attributes
                                                    .iter()
                                                    .filter(|x| INST_REG.contains(&x.as_str()) || 
                                                                          public_inputs.hcc_labels.contains_key(*x))
                                                    .map(|x| format!("{}_{}", model, x))
                                                    .collect::<Vec<String>>();

    raf_keys.iter_mut().for_each(|x| {
        if global_map.contains_key(x) {
            inst_reg_score += global_map.get(x).unwrap().coefficient;
            global_map.get_mut(x).unwrap().is_true = true;
        }
    });

    return inst_reg_score;
}

fn _get_new_enrollee_score(model: String, all_raf_attributes: &Vec<String>) -> f32 {
    let global_map = _get_global_raf_map();
    let mut new_enrollee_score = 0.0;
    let new_enrollee_vars = build_ne_reg_variable_list();

    let mut raf_keys = all_raf_attributes
                                                    .iter()
                                                    .filter(|x| new_enrollee_vars.contains(*x))
                                                    .map(|x| format!("{}_{}", model, x))
                                                    .collect::<Vec<String>>();

    raf_keys.iter_mut().for_each(|x| {
        if global_map.contains_key(x) {
            new_enrollee_score += global_map.get(x).unwrap().coefficient;
            global_map.get_mut(x).unwrap().is_true = true;
        }
    });

    return new_enrollee_score;    
}

pub fn main() {

    log("In Guest code main function");

    let _public_inputs: PublicRAFInputs = env::read();
    
    log("Retrieved public inputs");

    // Read in private inputs
    let _private_input: PrivateRAFInput = env::read();

    log("Retrieved private input");

    // Iterate through hcc_coefficients and initialize the GLOBAL_RAF_MAP
    let global_raf_map = _get_global_raf_map();
    for ele in _public_inputs.hcc_coefficients.iter() {
        let label = String::from(ele.0);
        global_raf_map.entry(label).or_insert(RAFAttribute {
            coefficient: *ele.1,
            is_true: false,
        });
    }

    log("Initialized global raf map");
    
    // Filter the private input diagnosis codes to only those that are mapped to HCCs
    let mut hcc_list = vec![];
    for dx in &_private_input.diagnosis_codes {
        if _public_inputs.dx_to_cc.contains_key(dx) {
            hcc_list.push(_public_inputs.dx_to_cc.get(dx).unwrap().clone());
        }
    }
    let flattened_hcc_list = hcc_list.into_iter().flatten().collect::<Vec<String>>();

    log("Got flattened HCC list");

    // Apply Age & Sex edits 
    let mut _age_sex_map = _age_sex_v2(_private_input.age, &_private_input.sex, &_private_input.entitlement_reason_code);
    if _private_input.long_term_institutionalized {
        _age_sex_map.push(String::from("LTIMCAID"));
    }

    log("Got age sex map ");

    // TODO: Apply ICD-10 edits (MCE data should be an input parameter)

    // Apply hierarchy to HCC list
    let _final_hcc_list = _apply_hierarchy(&_public_inputs.hcc_hierarchies, &flattened_hcc_list);

    log("Applied hierarchy to HCC list");

    // Apply interactions to HCC list
    let _final_interactions = _apply_interactions(&_final_hcc_list, &_private_input.entitlement_reason_code == "0");

    log("Applied interactions to HCC list");

    log("Collect all RAF attributes into a single list");
    let mut all_raf_attributes = vec![];
    all_raf_attributes.extend(_age_sex_map.iter().cloned());
    all_raf_attributes.extend(_final_hcc_list.iter().cloned());
    all_raf_attributes.extend(_final_interactions.iter().cloned());

    // Apply coefficients for each scoring model
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_NA,  RLIST=&COMM_REGA, CPREF=CNA_);
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_ND,  RLIST=&COMM_REGD, CPREF=CND_);
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_FBA, RLIST=&COMM_REGA, CPREF=CFA_);
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_FBD, RLIST=&COMM_REGD, CPREF=CFD_);
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_PBA, RLIST=&COMM_REGA, CPREF=CPA_);
    // %&SCOREMAC(PVAR=SCORE_COMMUNITY_PBD, RLIST=&COMM_REGD, CPREF=CPD_);

    let community_na_score = _get_community_model_reg_a_score("CNA".to_string(), &_private_input, &_public_inputs, &all_raf_attributes); 
    let community_nd_score = _get_community_model_reg_d_score("CND".to_string(), &_public_inputs, &all_raf_attributes);
    let community_fba_score = _get_community_model_reg_a_score("CFA".to_string(), &_private_input, &_public_inputs, &all_raf_attributes);
    let community_fbd_score = _get_community_model_reg_d_score("CFD".to_string(), &_public_inputs, &all_raf_attributes);
    let community_pba_score = _get_community_model_reg_a_score("CPA".to_string(), &_private_input, &_public_inputs, &all_raf_attributes);
    let community_pbd_score = _get_community_model_reg_d_score("CPD".to_string(), &_public_inputs, &all_raf_attributes);

    log("Got community model scores");

    // %&SCOREMAC(PVAR=SCORE_INSTITUTIONAL, RLIST=&INST_REG, CPREF=INS_);
    // %&SCOREMAC(PVAR=SCORE_NEW_ENROLLEE, RLIST=&NE_REG, CPREF=NE_);
    // %&SCOREMAC(PVAR=SCORE_SNP_NEW_ENROLLEE, RLIST=&NE_REG, CPREF=SNPNE_);

    let institutional_score = _get_institutional_reg_score("INS".to_string(), &_public_inputs, &all_raf_attributes);
    let new_enrollee_score = _get_new_enrollee_score("NE".to_string(), &all_raf_attributes);
    let snp_new_enrollee_score = _get_new_enrollee_score("SNPNE".to_string(), &all_raf_attributes);

    log("Got institutional scores, new enrollee scores, and SNP new enrollee scores");

    // Normalize the scores
    let mut all_raf_scores = HashMap::<String, f32>::new();
    all_raf_scores.insert("SCORE_COMMUNITY_NA".to_string(), community_na_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_COMMUNITY_ND".to_string(), community_nd_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_COMMUNITY_FBA".to_string(), community_fba_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_COMMUNITY_FBD".to_string(), community_fbd_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_COMMUNITY_PBA".to_string(), community_pba_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_COMMUNITY_PBD".to_string(), community_pbd_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_INSTITUTIONAL".to_string(), institutional_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_NEW_ENROLLEE".to_string(), new_enrollee_score * _public_inputs.norm_factor);
    all_raf_scores.insert("SCORE_SNP_NEW_ENROLLEE".to_string(), snp_new_enrollee_score * _public_inputs.norm_factor);

    log("Normalized scores, creating journal");

    // Calculate RAF score by summing the values for each HCC
    let journal = Journal {
        raf_scores: all_raf_scores,
        coefficients: HashMap::<String, f32>::new(),
    };

    log("Created journal, committing to environment");

    env::commit(&journal);

}


#[test]
fn can_apply_interactions() {
    let mut hcc_list = vec!["HCC21".to_string(), "HCC38".to_string(), "HCC221".to_string(), "HCC139".to_string()];
    let mut is_disabled = true;
    let first_interactions = _apply_interactions(&hcc_list, is_disabled);

    // CANCER_V28_DISABL should be present
    assert!(first_interactions.contains(&"DISABLED_CANCER_V28".to_string()));
    assert!(first_interactions.contains(&"DIABETES_HF_V28".to_string()));
    assert!(!first_interactions.contains(&"gSubUseDisorder_gPsych_V28".to_string()));
    assert!(first_interactions.contains(&"D4".to_string()));

    hcc_list = vec!["HCC21".to_string(), "HCC198".to_string(), "HCC221".to_string(), "HCC139".to_string()];
    is_disabled = false;
    let second_interactions = _apply_interactions(&hcc_list, is_disabled);
    assert!(!second_interactions.contains(&"gSubUseDisorder_gPsych_V28".to_string()));
    assert!(!second_interactions.contains(&"DISABLED_CANCER_V28".to_string()));
}

#[test]
fn can_apply_hierarchy() {
    let mut hiers = HashMap::<String, Vec<String>>::new();
    hiers
        .entry("HCC154".to_string())
        .or_insert(vec!["HCC155".to_string()]);
    hiers
        .entry("HCC180".to_string())
        .or_insert(vec!["HCC181".to_string(),
                                "HCC182".to_string(),
                                "HCC253".to_string(),
                                "HCC254".to_string()]);
    hiers
        .entry("HCC17".to_string())
        .or_insert(vec!["HCC18".to_string(),
                                "HCC19".to_string(),
                                "HCC20".to_string(),
                                "HCC21".to_string(),
                                "HCC22".to_string(),
                                "HCC23".to_string()]);    

    hiers 
        .entry("HCC222".to_string())
        .or_insert(vec!["HCC223".to_string(),
                                "HCC224".to_string(),
                                "HCC225".to_string(),
                                "HCC226".to_string(),
                                "HCC227".to_string()]);

    let hcc_list = vec!["HCC154".to_string(), "HCC155".to_string(), "HCC17".to_string(), "HCC19".to_string()];
    let final_hcc_list = _apply_hierarchy(&hiers, &hcc_list);
    assert!(final_hcc_list.contains(&"HCC154".to_string()));
    assert!(!final_hcc_list.contains(&"HCC155".to_string()));
    assert!(final_hcc_list.contains(&"HCC17".to_string()));
    assert!(!final_hcc_list.contains(&"HCC19".to_string()));


}

#[test]
fn can_build_age_sex_map() {
    let mut age = 65;
    let mut sex = "M";
    let mut orec = "1";
    let mut age_sex_map = _age_sex_v2(age, sex, orec);

    assert_eq!(age_sex_map.len(), 3);

    assert!(age_sex_map.contains(&String::from("M65_69"))); // Should be true
    assert!(!age_sex_map.contains(&String::from("F65_69"))); // Should be false
    assert!(age_sex_map.contains(&String::from("NEM65"))); // Should be true

    age = 64;
    sex = "F";
    orec = "1";
    age_sex_map = _age_sex_v2(age, sex, orec);

    assert!(age_sex_map.contains(&String::from("F60_64"))); // Should be true
    assert!(!age_sex_map.contains(&String::from("NEF65"))); // Should be false
    assert!(age_sex_map.contains(&String::from("NEF60_64"))); // Should be true


    orec = "0";
    age_sex_map = _age_sex_v2(age, sex, orec);

    assert!(age_sex_map.contains(&String::from("NEF65"))); // Should be true


}

#[test]
fn can_generate_community_model_a_score() {

    let _private_input = PrivateRAFInput {
        age: 75,
        diagnosis_codes: vec![],
        sex: String::from("M"),
        eligibility_code: String::from("0"),
        entitlement_reason_code: String::from("0"),
        medicaid_status: false,
        long_term_institutionalized: false,
    };

    let all_raf_attributes: Vec<String> = vec!["M75_79".to_string(), "NEM75_79".to_string(), "DIABETES_HF_V28".to_string(), "D3".to_string()];

    let mut hcc_coefficients = HashMap::<String, f32>::new();
    hcc_coefficients.insert("CNA_M75_79".to_string(), 0.50);
    hcc_coefficients.insert("CNA_NEM75_79".to_string(), 0.0);
    hcc_coefficients.insert("CNA_DIABETES_HF_V28".to_string(), 0.11);
    hcc_coefficients.insert("CNA_D3".to_string(), 0.0);
    hcc_coefficients.insert("CNA_HF_HCC238_V28".to_string(), 0.08);
    hcc_coefficients.insert("CNA_HCC379".to_string(), 1.97);
    hcc_coefficients.insert("CNA_HCC380".to_string(), 1.08);
    hcc_coefficients.insert("CNA_HCC381".to_string(), 1.08);
    hcc_coefficients.insert("CNA_HCC382".to_string(), 0.84);       

    let mut hcc_labels = HashMap::<String, String>::new(); 
    hcc_labels.insert("HCC1".to_string(), "HIV/AIDS".to_string());
    hcc_labels.insert("HCC2".to_string(), "Septicemia, Sepsis, Systemic Inflammatory Response Syndrome/Shock".to_string());
    hcc_labels.insert("HCC6".to_string(), "Opportunistic Infections".to_string());
    hcc_labels.insert("HCC17".to_string(), "Cancer Metastatic to Lung, Liver, Brain, and Other Organs; Acute Myeloid Leukemia Except Promyelocytic ".to_string());
    hcc_labels.insert("HCC18".to_string(), "Cancer Metastatic to Bone, Other and Unspecified Metastatic Cancer; Acute Leukemia Except Myeloid ".to_string());
    hcc_labels.insert("HCC19".to_string(), "Myelodysplastic Syndromes, Multiple Myeloma, and Other Cancers ".to_string());
    hcc_labels.insert("HCC21".to_string(), "Lymphoma and Other Cancers ".to_string());
    hcc_labels.insert("HCC22".to_string(), "Bladder, Colorectal, and Other Cancers ".to_string());
    hcc_labels.insert("HCC23".to_string(), "Prostate, Breast, and Other Cancers and Tumors ".to_string());
    hcc_labels.insert("HCC20".to_string(), "Lung and Other Severe Cancers ".to_string());
    hcc_labels.insert("HCC35".to_string(), "Pancreas Transplant Status".to_string());
    hcc_labels.insert("HCC36".to_string(), "Diabetes with Severe Acute Complications".to_string());
    hcc_labels.insert("HCC37".to_string(), "Diabetes with Chronic Complications".to_string());
    hcc_labels.insert("HCC38".to_string(), "Diabetes with Glycemic, Unspecified, or No Complications ".to_string());
    hcc_labels.insert("HCC48".to_string(), "Morbid Obesity".to_string());

    let mut hiers = HashMap::<String, Vec<String>>::new();
    hiers
        .entry("HCC154".to_string())
        .or_insert(vec!["HCC155".to_string()]);

    let mut dx_to_cc = HashMap::<String, Vec<String>>::new();
    dx_to_cc
        .entry("B20".to_string())
        .or_insert(vec!["HCC1".to_string(), "HCC6".to_string()]);

    let _public_input = PublicRAFInputs {
        hcc_coefficients: hcc_coefficients,
        hcc_labels: hcc_labels,
        hcc_hierarchies: hiers,
        dx_to_cc: dx_to_cc,
        norm_factor: 1.0,
    };

    let global_raf_map = _get_global_raf_map();
    for ele in _public_input.hcc_coefficients.iter() {
        let label = String::from(ele.0);
        global_raf_map.entry(label).or_insert(RAFAttribute {
            coefficient: *ele.1,
            is_true: false,
        });
    }

    let score_community_na = _get_community_model_reg_a_score("CNA".to_string(), &_private_input, &_public_input, &all_raf_attributes);

    assert_eq!(score_community_na, 0.61);

}
