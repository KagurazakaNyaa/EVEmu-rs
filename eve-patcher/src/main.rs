use std::{
    env,
    fs::{self, File},
    io::Read,
};

// Only need version 360229 https://github.com/stschake/blue_patcher/blob/f16ba3ceb3c439fd42f5b3fe0abf241ca60828bc/Source/blue_patcher/Patcher.cs#L16C47-L16C47
static PATTERN: &[u8] = &[
    0x6a, 0x00, 0x6a, 0x00, 0x52, 0x2b, 0xc1, 0x50, 0x8b, 0x85, 0xc8, 0xfa, 0xff, 0xff, 0x51, 0x50,
    0xc6, 0x85, 0xcf, 0xfa, 0xff, 0xff, 0x01, 0xff, 0x15, 0x00, 0x80, 0x15, 0x10, 0x8b, 0x8d, 0xc8,
    0xfa, 0xff, 0xff, 0x51, 0x8b, 0xf0, 0xff, 0x15, 0x1C, 0x80, 0x15, 0x10, 0x85, 0xf6, 0x0f, 0x85,
];
static MASK: &[bool] = &[
    true, true, true, false, true, true, false, false, true, true, false, false, true, true, true,
    true, true, true, true, false, false, false, false, true, true, true, true, false, false,
    false, false, true, true, true, true, false, false, true, true, true, false, false, false,
    false, true, true, true, true,
];

static TARGET: &[u8] = &[0x90, 0xE9]; // nop, jmp

// return offset for 0x0f 0x85 (jnz)
fn search(content: &Vec<u8>) -> Option<usize> {
    let mut i: usize = 0;
    let length: usize = PATTERN.len();
    for byte in content {
        if byte == &PATTERN[0] {
            let mut is_match = true;
            let mut iter: usize = 0;
            if i + length > content.len() {
                break;
            }
            let part = &content[i..i + length];
            for b in part {
                if MASK[iter] && &PATTERN[iter] != b {
                    is_match = false;
                    break;
                }
                iter += 1;
            }
            if is_match {
                return Some(i + PATTERN.len() - 3);
            }
        }
        i += 1;
    }
    None
}

fn patch(content: Vec<u8>) -> Option<Vec<u8>> {
    if let Some(offset) = search(&content) {
        println!("find pattern at offset {:#08x}", offset);
        let mut result = content.clone();
        let mut ret_offset = offset + 1;
        for byte in TARGET {
            result[ret_offset] = byte.clone();
            ret_offset += 1;
        }
        return Some(result);
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bin_file = &args[1];
    let bak_file = format!("{bin_file}.bak");
    let mut f = File::open(bin_file).unwrap();
    let mut bts: Vec<u8> = vec![];
    f.read_to_end(&mut bts).unwrap();
    if let Some(new) = patch(bts.clone()) {
        fs::write(bak_file, &bts).unwrap();
        fs::write(bin_file, &new).unwrap();
        println!("blue.dll patched!");
    } else {
        println!("cannot find any pattern match!")
    }
}
