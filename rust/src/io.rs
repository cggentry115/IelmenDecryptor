use std::io::Write;
use std::path::Path;
use std::fs;

pub fn write_file(f_name : &String, dir_out : &String, f_result : Vec<u8>) {
    let pos = f_name.rfind("www").unwrap();
    let out_file = dir_out.to_owned() + "/" + &f_name[pos..];
    let path = Path::new(&out_file).parent().unwrap();
    fs::create_dir_all(path).unwrap();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&out_file).unwrap();

    file.write_all(&f_result).unwrap();

    println!("single file decrypted and saved at: {}", &out_file);
}
