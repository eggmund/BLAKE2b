// Gets a single 64 bit word from list of 8 bytes
#[inline]
pub fn get_64(inp: &[u8]) -> u64 {
    inp[0] as u64 ^
    ((inp[1] as u64) << 8) ^
    ((inp[2] as u64) << 16) ^
    ((inp[3] as u64) << 24) ^
    ((inp[4] as u64) << 32) ^
    ((inp[5] as u64) << 40) ^
    ((inp[6] as u64) << 48) ^
    ((inp[7] as u64) << 56)
}

// gets 8 bytes from a single u64. Inverse of get_64
#[inline]
fn get_8(inp: &u64) -> [u8; 8] {
    let mut out = [0u8; 8];

    for i in (0usize..64).step_by(8) {
        out[i/8] = (inp >> i) as u8;
    }

    out
}

// Gets final hash in little endian form
pub fn get_little_endian_string(h: &[u64; 8]) -> String {
    let mut out = String::new();

    for num in h.iter() {
        let eight = get_8(num);
        for sub in eight.iter() {
            out += format!("{:x}", sub).as_str();
        }
        out += " ";
    }

    out
}

pub fn get_little_endian_string_16(h: &[u64; 16]) -> String {
    let mut out = String::new();

    for num in h.iter() {
        let eight = get_8(num);
        // reverse because little endian
        for sub in eight.iter().rev() {
            out += format!("{:x}", sub).as_str();
        }
        out += " ";
    }

    out
}