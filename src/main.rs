use std::fs::File;
use r#box::{size_read, type_read, box_read, box_count};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = String::from(r"C:\Users\Mint\Downloads\weekend.mp4");
    let mut fil = File::open(arg)?;
    
    let size = size_read(&mut fil, 0);
    let _ = type_read(&mut fil, 0);
    let fytp_box = box_read(&mut fil, size.expect("no size"), 0);
    println!("{fytp_box:?}");
    let count = box_count(&mut fil)?;
    println!("{} box count total", count);
    println!("finished");
    Ok(())
}




