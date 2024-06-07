use walkdir::WalkDir;

/*
    * This function is used to find files in a folder.
    * @param path: &str
    * @return ()
*/
pub fn flag_a(path: &str) -> () {
    let walk: WalkDir = WalkDir::new(path).min_depth(1).max_depth(1);
    let entries: Vec<_> = walk.into_iter().collect();

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
