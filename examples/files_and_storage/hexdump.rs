use std::{fs::File, io::Read};

const BYTES_PER_LINE: usize = 16;
/// `b` indicates that this should be treated as individual bytes, and not UTF-8 text
/// Double quotes don't need to be escaped when using raw string literals.
const INPUT: &'static [u8] = br#"
fn main(){
    println!("Hello, world!");
}"#;
fn main() {
    file_read();
    // string_read().unwrap();
}

fn file_read() {
    // Get the second argument. First argument, at 0 index, is always program name
    let arg1 = std::env::args().nth(1);
    // Get file name
    let file_name = arg1.expect("usage: fview FILENAME");
    // Open file
    let mut f = File::open(&file_name).expect("Unable to open file");
    let mut pos = 0;
    // Fixed sized buffer
    let mut buffer = [0; BYTES_PER_LINE];

    // Read exactly 16 bytes, only to fill buffer. It will stop reading when buffer is full
    // read_exact gives greater control than chunks method, but has some quirks
    // If buffer is longer than number of available bytes to read, the file will return an
    // error and buffer's state will be undefined.
    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{pos:08x}]");
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{byte:02x} "),
            }
        }
        println!();
        pos += BYTES_PER_LINE;
    }
}

#[allow(dead_code)]
fn string_read() -> std::io::Result<()> {
    // Makes space for the program's input with internal buffer
    let mut buffer = vec![];
    // Reads the input and insert into buffer
    #[allow(const_item_mutation)]
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        // Writes the current position with upto 8 left-padded zeros
        print!("[0x{position_in_input:08x}]");

        for byte in line {
            print!("{byte:02x} ");
        }

        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
