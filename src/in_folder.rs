use walkdir::WalkDir;
use walkdir::DirEntry;

/*
    * This function is used to filter hidden files and directories.
    * @param entry: &DirEntry
    * @return bool
*/
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/*
    * This function is used to find files in a folder.
    * @param path: &str
    * @return ()
*/
pub fn find_files_in_folder(path: &str) -> () {
    let walk: WalkDir = WalkDir::new(path).min_depth(1).max_depth(1);
    let entries: Vec<_> = walk.into_iter().filter_entry(|e|
        !is_hidden(e)).collect();

    for (i, entry) in entries.iter().enumerate() {
        match entry {
            Ok(entry) => {
                if i == entries.len() - 1 {
                    println!("{}",
                        entry.path().strip_prefix(path).unwrap().display())
                } else {
                    print!("{} ",
                        entry.path().strip_prefix(path).unwrap().display())
                    // strip_prefix() is used to remove the path prefix
                }
            },
            Err(e) => eprintln!("Error => {}", e),
            // Return error message on the standard error output
        }
    }
}
