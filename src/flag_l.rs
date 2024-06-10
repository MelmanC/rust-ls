use walkdir::WalkDir;
use crate::in_folder::is_hidden;
use std::{fs::{self, Metadata}, os::macos::fs::MetadataExt, path::Display, time::SystemTime};
use chrono::prelude::*;
use file_owner::PathExt;
use std::os::unix::fs::PermissionsExt;
use crate::permissions::get_permisions;

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
                let owner = entry.path().owner().unwrap();
                let group = entry.path().group().unwrap();
                let permisions = entry.path().metadata().unwrap().permissions();
                let mode = permisions.mode();
                let mode = mode & 0o777;
                let mode = format!("{:o}", mode);
                let string_permisions = get_permisions(mode, entry.path());
                println!("{} {} {} {} {} {} {} {}", string_permisions, owner, group, metadata.len(), day, month, formatted_time, path);
            },
            Err(e) => eprintln!("Error => {}", e),
            // Return error message on the standard error output
        }
    }
}
