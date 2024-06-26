use std::env;
mod in_folder;
mod flag_a;
mod flag_l;
mod permissions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len < 2 {
        eprintln!("Usage: [-a] [-l] [-R] <path>");
        return;
    }
    if args[1] == "-a" {
        if args_len == 3 {
            flag_a::flag_a(&args[2]);
        } else {
            flag_a::flag_a(".");
        }
        return;
    }
    if args[1] == "-l" {
        if args_len == 3 {
            flag_l::flag_l(&args[2]);
        } else {
            flag_l::flag_l(".");
        }
        return;
    }
    if args_len == 1 {
        in_folder::find_files_in_folder(".");
    }
    if args_len == 2 {
        in_folder::find_files_in_folder(&args[1]);
    }
}
