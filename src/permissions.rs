use std::path::Path;

/*
    * This function is used to get the last modification date of a file.
    * @param metadata: Metadata
    * @return String
    * NOTE : Change this function to avoid repetition
*/
pub fn get_permisions(mode: String, path: &Path) -> String {
    let mut permisions = String::new();
    let mode = mode.chars().collect::<Vec<char>>();
    let user = mode[0];
    let group = mode[1];
    let other = mode[2];

    if path.is_dir() {
        permisions.push_str("d");
    } else {
        permisions.push_str("-");
    }
    match user {
        '7' => permisions.push_str("rwx"),
        '6' => permisions.push_str("rw-"),
        '5' => permisions.push_str("r-x"),
        '4' => permisions.push_str("r--"),
        '3' => permisions.push_str("-wx"),
        '2' => permisions.push_str("-w-"),
        '1' => permisions.push_str("--x"),
        '0' => permisions.push_str("---"),
        _ => permisions.push_str("Unknown"),
    }
    match group {
        '7' => permisions.push_str("rwx"),
        '6' => permisions.push_str("rw-"),
        '5' => permisions.push_str("r-x"),
        '4' => permisions.push_str("r--"),
        '3' => permisions.push_str("-wx"),
        '2' => permisions.push_str("-w-"),
        '1' => permisions.push_str("--x"),
        '0' => permisions.push_str("---"),
        _ => permisions.push_str("Unknown"),
    }
    match other {
        '7' => permisions.push_str("rwx"),
        '6' => permisions.push_str("rw-"),
        '5' => permisions.push_str("r-x"),
        '4' => permisions.push_str("r--"),
        '3' => permisions.push_str("-wx"),
        '2' => permisions.push_str("-w-"),
        '1' => permisions.push_str("--x"),
        '0' => permisions.push_str("---"),
        _ => permisions.push_str("Unknown"),
    }
    permisions
}
