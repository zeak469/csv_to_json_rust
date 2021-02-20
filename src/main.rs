use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::process;
use std::env;

fn read_csv_file(filename: String) -> Result<String, Box<dyn Error>> {

    let mut all_json = json::JsonValue::new_array();
    let mut json_object;

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(File::open(filename)?);
    for result in rdr.records() {
        let result_unboxed = result.unwrap();
        json_object = json::object!{};
        let mut d1 = json::JsonValue::new_array();
        json_object["question"] = result_unboxed.get(0).into();
        for i in 1..=5{
            if i > 1{
                let _e = d1.push(result_unboxed.get(i));
            }
            else{
                json_object["right_answer"] = result_unboxed.get(1).into();
            }
        }
        json_object["answers"] = d1;
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let _t = all_json.push(json_object);
    }
    Ok(all_json.dump())
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    let file_name_path = String::from(&args[1]);
    println!("\nOpening {}...\n", file_name_path);
    match read_csv_file(file_name_path) {
        Ok(json_string) =>{
            // Create file
            let mut file = File::create("questions.json")?;
            file.write_all(json_string.as_bytes())?;
            Ok(())
        }
        Err(err) => {
            println!("ERROR {}", err);
            process::exit(1);
        }
    }
}
