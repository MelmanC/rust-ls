use walkdir::WalkDir;
use crate::in_folder::is_hidden;
use std::{fs::{self, Metadata}, path::Display, time::SystemTime};
use chrono::prelude::*;

fn translate_month(str: &str) -> String {
    match str {
        "Jan" => "jan".to_string(),
        "Feb" => "fev".to_string(),
        "Mar" => "mar".to_string(),
        "Apr" => "apr".to_string(),
        "May" => "mai".to_string(),
        "Jun" => "jui".to_string(),
        "Jul" => "jui".to_string(),
        "Aug" => "aou".to_string(),
        "Sep" => "sep".to_string(),
        "Oct" => "oct".to_string(),
        "Nov" => "nov".to_string(),
        "Dec" => "dec".to_string(),
        _ => "Unknown".to_string(),
    }
}

/*
    * This function is used to find files in a folder.
    * @param path: &str
    * @return ()
*/
pub fn flag_l(path: &str) -> () {
    let walk: WalkDir = WalkDir::new(path).min_depth(1).max_depth(1);

    for entry in walk.into_iter().filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(entry) => {
                let path: Display =
                    entry.path().strip_prefix(path).unwrap().display();
                let metadata: Metadata = fs::metadata(entry.path()).unwrap();
                let time: SystemTime = metadata.modified().unwrap();
                let time: DateTime<Local> = time.into();
                let day: u32 = time.day();
                let mut month: String = time.format("%b").to_string();
                month = translate_month(&month);
                let formatted_time: String = time.format("%H:%M").to_string();

                println!("{} {} {} {} {}", metadata.len(), day, month, formatted_time, path);
            },
            Err(e) => eprintln!("Error => {}", e),
            // Return error message on the standard error output
        }
    }
}
