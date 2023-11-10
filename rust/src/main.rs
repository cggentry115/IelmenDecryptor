mod decrypt;
mod mask;
mod io;

use std::fs;
use std::env;
use std::path::Path;
use std::fs::metadata;

use decrypt::{decrypt_folder, decrypt_file};
use io::write_file;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 1 && args.len() != 3 {
        println!("USAGE | ielmen_decryptor [in] [out]");
        println!("[in]  | input directory or file");
        println!("      | directory is traversed recursively");
        println!("[out] | output directory"); 
        println!("      | location of decrypted files");
        println!("| you may also supply no arguments to use the default file paths");
        std::process::exit(-1);
    }

    let mut in_path = String::from("../game");
    let mut out_path = String::from("./decrypted");

    if args.len() == 3 {
        in_path = args[1].to_owned();
        out_path = args[2].to_owned();
    }

    let meta_in = metadata(Path::new(&in_path)).unwrap();
    
    let in_exist = Path::new(&in_path).exists();
    if !in_exist {
        if args.len() == 1 {
            println!("try moving this executable or supplying input parameters");
            println!("running without parameters requires being in the 'game' directory");
        }
        else {
            println!("invalid input directory: {}", in_path);
        }

        std::process::exit(-1);
    }
    else {
        if meta_in.is_dir() {
            if args.len() == 1 {
                let game_file = in_path.to_owned() + "/Game.exe";
                let in_game_dir : bool = Path::new(&game_file).exists();
                if !in_game_dir {
                    println!("non-game directory detected");
                    std::process::exit(-1);
                }
            }
        }
    }

    let out_exist = Path::new(&out_path).exists();
    if !out_exist {
        if args.len() == 1 {
            fs::create_dir(&out_path).unwrap();
        }
        else {
            println!("invalid output directory: {}", out_path);
            std::process::exit(-1);
        }
    }

    if meta_in.is_dir() {
        decrypt_folder(&in_path, &out_path);

        println!("directory processed: {}", out_path);
    }
    else if meta_in.is_file() {
        let (f_result, f_flag) = decrypt_file(&in_path);
        if !f_flag {
            write_file(&in_path, &out_path, f_result);
        }
    }
}
