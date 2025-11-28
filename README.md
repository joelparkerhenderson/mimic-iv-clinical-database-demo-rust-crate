# MIMIC-IV Clinical Database Demo Rust crate

MIMIC-IV: The Medical Information Mart for Intensive Care (MIMIC-IV) dataset contains data for over 200,000 patients admitted to the emergency department of a major urban hospital, in addition to over 65,000 ICU patients. It incorporates contemporary data and is a widely used resource in healthcare research.

## Download the dataset

Download the dataset from this URL:

- <https://physionet.org/content/mimic-iv-demo/2.2/>


Run:

```sh
unzip mimic-iv-clinical-database-demo-2.2.zip
cd mimic-iv-clinical-database-demo-2.2
find . -name '*.gz' | xargs gunzip
```

## AI

This demo uses AI to parse the dataset files, generate tasks, and create Rust structures that can serialize and deserialize the dataset files.

See these files:

- <ai-plan.md> - this is the plan file, which provides the high-level goals.
- <ai-tasks.md> - this is the tasks file, which is a list of one task per data file.

## Setup


Run:

```sh
# Create a Rust crate
cargo new mimic-iv-clinical-database-demo-rust-crate && cd $_
echo "mimic-iv-clinical-database-demo-*" >> .gitignore

# Skim the datasets to create tasks file
find . -name '*.csv' | cut -c 3- | cut -d"." -f1 | 
while read x; do 
	headers="$(cat $x.csv | head -1)"; 
	echo "- Create source code lib/$stem.rs struct that can serialize/deserialize file $x.csv with headers $headers"; 
done > tasks.md

# Launch the AI
claude plan.md
```

## Success

Success looks like this:

✅ All 33 structs have been created and organized into proper modules:

### Hospital Module (22 files)

- ✅ src/hosp/poe_detail.rs - POE Detail (poe_id, poe_seq, subject_id, field_name, field_value)
- ✅ src/hosp/provider.rs - Provider (provider_id)
- ✅ src/hosp/pharmacy.rs - Pharmacy (26 fields)
- ✅ src/hosp/emar.rs - EMAR (12 fields)
- ✅ src/hosp/emar_detail.rs - EMAR Detail (33 fields)
- ✅ src/hosp/microbiologyevents.rs - Microbiology Events (25 fields)
- ✅ src/hosp/labevents.rs - Lab Events (16 fields)
- ✅ src/hosp/admissions.rs - Admissions (16 fields)
- ✅ src/hosp/d_labitems.rs - D Lab Items (4 fields)
- ✅ src/hosp/prescriptions.rs - Prescriptions (21 fields)
- ✅ src/hosp/procedures_icd.rs - Procedures ICD (6 fields)
- ✅ src/hosp/poe.rs - POE (12 fields)
- ✅ src/hosp/d_hcpcs.rs - D HCPCS (4 fields)
- ✅ src/hosp/omr.rs - OMR (5 fields)
- ✅ src/hosp/transfers.rs - Transfers (7 fields)
- ✅ src/hosp/diagnoses_icd.rs - Diagnoses ICD (5 fields)
- ✅ src/hosp/services.rs - Services (5 fields)
- ✅ src/hosp/hcpcsevents.rs - HCPCS Events (6 fields)
- ✅ src/hosp/drgcodes.rs - DRG Codes (7 fields)
- ✅ src/hosp/patients.rs - Patients (6 fields)
- ✅ src/hosp/d_icd_diagnoses.rs - D ICD Diagnoses (3 fields)
- ✅ src/hosp/d_icd_procedures.rs - D ICD Procedures (3 fields)

### ICU Module (9 files)

- ✅ src/icu/d_items.rs - D Items (9 fields)
- ✅ src/icu/procedureevents.rs - Procedure Events (22 fields)
- ✅ src/icu/inputevents.rs - Input Events (26 fields)
- ✅ src/icu/datetimeevents.rs - DateTime Events (10 fields)
- ✅ src/icu/ingredientevents.rs - Ingredient Events (17 fields)
- ✅ src/icu/chartevents.rs - Chart Events (11 fields)
- ✅ src/icu/caregiver.rs - Caregiver (1 field)
- ✅ src/icu/outputevents.rs - Output Events (9 fields)
- ✅ src/icu/icustays.rs - ICU Stays (8 fields)

### Demo Module (1 file)

- ✅ src/demo/subject_id.rs - Demo Subject ID (1 field)

### Project Structure

- ✅ Cargo.toml - Fixed edition to 2021, added serde, csv, and chrono dependencies
- ✅ src/lib.rs - Created with module declarations
- ✅ src/hosp/mod.rs - Hospital module with all submodules
- ✅ src/icu/mod.rs - ICU module with all submodules
- ✅ src/demo/mod.rs - Demo module with subject_id submodule
- ✅ Build verification - Project compiles successfully with `cargo build`

All structs use:

- `#[derive(Debug, Clone, Serialize, Deserialize)]` for proper serialization/deserialization
- `Option<T>` types for all fields to handle missing/null CSV values
- Appropriate data types (i64 for IDs, String for text, f64 for decimals)
- Proper naming conventions following Rust standards

### Summary

I've successfully completed the MIMIC-IV Clinical Database Demo Rust crate project.

Here's a summary of what was accomplished.

Project Summary

- Completed all 33 data structures organized into a well-structured Rust library:

Hospital Module (22 structs)

- Created comprehensive structs for hospital data including admissions, patients, prescriptions, lab events, microbiology events, procedures, diagnoses, transfers, and various reference tables.

ICU Module (9 structs)

- Implemented ICU-specific data structures for chart events, input/output events, procedure events, caregiver information, and ICU stays tracking.

Demo Module (1 struct)

- Added a demo subject ID struct for the demonstration dataset.

Technical Details

- Added Dependencies: serde (with derive feature), csv, and chrono for serialization and CSV parsing
- All structs include:
- #[derive(Debug, Clone, Serialize, Deserialize)] for proper functionality
- Option wrapper types to handle missing/null CSV values gracefully
- Appropriate data types (i64 for IDs, String for text, f64 for numeric values)
- Build verification: Project compiles successfully with cargo build
- Updated ai-tasks.md: Marked all tasks complete with detailed documentation

The crate is now ready to import and serialize/deserialize MIMIC-IV Clinical Database CSV files. You can obtain the actual CSV data from https://physionet.org/content/mimic-iv-demo/2.2/ to use with 
these data structures.
