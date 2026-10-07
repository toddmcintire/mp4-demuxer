use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct FYTPBox {
    size: u32,
    r#type: String,
    major_brand: String,
    minor_version: u32,
    comppatible_brands: Vec<String>
}

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

pub fn type_read(fil: &mut File, offset: u64) -> Result<String, std::io::Error> {
    //get start position
    let _ = fil.seek(SeekFrom::Start(4+offset));

    //read from start to specific location
    let mut type_buf = vec![0u8;4];
    fil.read_exact(&mut type_buf).expect("buf incorrect");
    //println!("{:?}", type_buf);

    let box_type = String::from_utf8(type_buf).expect("not vec of u8");
    println!("{} type", box_type);

    //convert to string
    Ok(box_type)
}

///test function to read ftyp box
// pub fn box_read(fil: &mut File, size: u32, offset: u64) -> FYTPBox{
//     let _ = fil.seek(SeekFrom::Start(offset));

//     let box_casted_size = size as usize;
//     let mut box_buf = vec![0u8; box_casted_size];
//     fil.read_exact(&mut box_buf).expect("buf incorrect");

//     //println!("{:?}", box_buf);

//     //take 4 for major brand
//     let brand_buf = box_buf[8..12].to_vec();
//     let major_brand = String::from_utf8(brand_buf).expect("not vec of u8");
//     println!("{:?}", major_brand);

//     //take 4 for minor version
//     let minor_buf = box_buf[12..16].to_vec();
//     let minor = u32::from_be_bytes(minor_buf.try_into().expect("box length incorrect"));
//     println!("{:?}", minor);

//     //rest for brands (4 bytes each)
//     let brands_buf = box_buf[16..].to_vec();
//     let mut comppatible_brands_buf: Vec<String> = vec![];

//     for chunk in brands_buf.chunks(4) {
//         println!("{chunk:?}");
//         let chunked = String::from_utf8(chunk.to_vec());
//         println!("{chunked:?}");
//         comppatible_brands_buf.push(chunked.expect("chunk not loaded"));

//     }

//     FYTPBox {size,r#type, major_brand, minor_version: minor, comppatible_brands: comppatible_brands_buf }
// }

pub fn box_count(fil: &mut File) -> Result<u64, std::io::Error> {
    //iterate through boxes getting total count.
    let comple_size = fil.metadata()?.len();
    let mut offset: u64 = 0;
    let mut count: u64 = 0;


    //println!("{}", comple_size);

    while offset != comple_size {
            let size = size_read(fil, offset)?;
            let resized:u64 = size.into();
            offset += resized;
            count += 1;
            println!("{offset}")
    }
    
    Ok(count)
}

enum ParsedBox {
    FYTP(FYTPBox),
    Unknown
}

///parses a box and returns box struct
fn box_parser(size: u32, r#type: String) -> ParsedBox{

    //ftyp
    if true {
        let mut ftyp = FYTPBox { 
            size,
            r#type, 
            major_brand: String::new(), 
            minor_version: 0, 
            comppatible_brands: vec![String::new()] 
        };

        return ParsedBox::FYTP(ftyp)
    } else {
        return ParsedBox::Unknown
    }
    

}