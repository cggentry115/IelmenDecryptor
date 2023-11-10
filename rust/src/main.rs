mod decrypt;
mod mask;

use std::env;
use std::path::Path;

use decrypt::decrypt_folder;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("USAGE | ielmen_decryptor [in] [out]");
        println!("[in]  | input directory or file");
        println!("      | directory is traversed recursively");
        println!("[out] | output directory"); 
        println!("      | location of decrypted files");
        std::process::exit(-1);
    }

    let in_exist = Path::new(&args[1]).exists();
    if !in_exist {
        println!("invalid input directory: {}", args[1]);
        std::process::exit(-1);
    }

    let out_exist = Path::new(&args[2]).exists();
    if !out_exist {
        println!("invalid output directory: {}", args[2]);
        std::process::exit(-1);
    }


    decrypt_folder(&args[1], &args[2]);


}
