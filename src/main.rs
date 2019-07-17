mod blake;

use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let out = blake::checksum::get_checksum(
        Path::new("./test_files/abc.txt"),
        64,
    )?;

    // let mut out = [0u64; 8];
    // out[0] = 0x000000000000636261;


    println!("{}", blake::tools::get_little_endian_string(&out));

    Ok(())
}