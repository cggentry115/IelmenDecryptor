use glob::glob;
use super::mask::*;
use super::io::*;
use std::fs;
use std::path::Path;

static SIGNATURE : &str = "00000NEMLEI00000";

//use std::fs;

pub fn decrypt_folder(dir_path : &String, dir_out : &String) {
    let glob_str : String = dir_path.to_owned() + "/**/";
    
    for entry in glob(&(glob_str.to_owned() + "img/*/*.png")).unwrap()
                .chain(glob(&(glob_str.to_owned() + "data/*.json")).unwrap())
                .chain(glob(&(glob_str.to_owned() + "audio/*/*.ogg")).unwrap()) {

        let mut f_name : String = "".to_string();

        match entry {
            Ok(path) => f_name = path
                                 .into_os_string()
                                 .into_string()
                                 .unwrap(),
            _ => ()
        }

        let (f_result, f_flag) = decrypt_file(&f_name);
        if !f_flag {
            write_file(&f_name, dir_out, f_result);
        }
    }
}

pub fn decrypt_file(f_path : &String) -> (Vec<u8>, bool) {
    let strip_path = Path::new(&f_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_uppercase(); 
    let mut mask_val : u32 = mask(&strip_path);
    let sig_len  : usize = SIGNATURE.len() as usize;

    let raw_data : Vec<u8> = fs::read(&f_path).unwrap();

    let sliced_input : Vec<u8> = raw_data[0..sig_len].to_vec();
    let mut decoded_chars : Vec<u8> = vec![0; sig_len];

    for i in 0..sig_len {
        let c : u32 = SIGNATURE.as_bytes()[i] as u32;
        let temp : u32 = (c ^ mask_val) % 0x100;
        decoded_chars[i] = temp as u8;
    }

    if decoded_chars != sliced_input {
        return (vec![0], true);
    }
    
    let mut rem_data : Vec<u8> = raw_data[sig_len+1..].to_vec();
    let zero_idx_val : u8 = raw_data[sig_len];
    let mut decrypt_len : usize = zero_idx_val as usize;

    if zero_idx_val == 0 {
        decrypt_len = rem_data.len() as usize;
    }

    for i in 0..decrypt_len {
        let temp : u8 = rem_data[i];
        let decrypt_byte : u32 = (rem_data[i] as u32 ^ mask_val) % 0x100;
        rem_data[i] = decrypt_byte as u8;
        mask_val = (mask_val << 1) ^ temp as u32;
    }

    let mut f_flag : bool = false;
    if rem_data.len() == 1 {
        println!("Decoded signature doesn't match file header! Is the key wrong or the input file not encrypted?");
        println!("Decryption aborted. File was NOT decrypted: {:}", f_path);
        f_flag = true;
    }

    return (rem_data, f_flag);
}
