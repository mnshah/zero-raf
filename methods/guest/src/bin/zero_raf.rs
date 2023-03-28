#![no_main]
// #![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;
use zero_raf_core::{PublicRAFInputs, PrivateRAFInput};
use std::collections::HashMap;


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

    let disabl = (age < 65 && orec != "0");
    age_sex_map.insert(String::from("DISABL"), disabl);
    age_sex_map.insert(String::from("ORIGDS"), (orec == "1" && !disabl));

    return age_sex_map;
}

pub fn main() {

    // Read in public inputs
    let _public_input: PublicRAFInputs = env::read();

    // Read in private inputs
    let _private_input: PrivateRAFInput = env::read();
    
    // Filter the private input diagnosis codes to only those that are mapped to HCCs
    let mut dx_list = vec![];
    for dx in _private_input.diagnosis_codes {
        if _public_input.dx_to_cc.contains_key(&dx) {
            dx_list.push(dx);
        }
    }

    // Apply Age & Sex edits 
    let age_sex_map = _age_sex_v2(_private_input.age, &_private_input.sex, &_private_input.entitlement_reason_code);

    // TODO: Apply ICD-10 edits (MCE data should be an input parameter)

    // Apply hierarchy to HCC list

    // Apply interactions to HCC list

    // Apply coefficients

    // Calculate RAF score by summing the values for each HCC

}
