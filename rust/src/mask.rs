
pub fn mask(in_str : &String) -> u32 {
    let mut mask_val : u32 = 0;
    for c in 0..in_str.len() {
        mask_val = (mask_val << 1) ^ in_str.as_bytes()[c] as u32;
    }
    return mask_val;
}
