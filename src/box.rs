use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn size_read(fil: &mut File, offset: u64) -> Result<u32, std::io::Error> {
    //get start position
    fil.seek(SeekFrom::Start(0+offset))?;

    //read from start to specific location
    let mut size_buf = vec![0u8;4];
    fil.read_exact(&mut size_buf).expect("buf incorrect");
    //println!("{:?}", size_buf);

    //convert to u32
    let size = u32::from_be_bytes(size_buf.try_into().expect("box length incorrect"));

    println!("{:?} size", size);
    Ok(size)
}