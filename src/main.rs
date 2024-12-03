use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

const BYTES_PER_LINE: usize = 16;

fn print_hexdump<R: Read>(reader: R) -> io::Result<()> {
    let mut buffer = BufReader::new(reader);
    let mut position_in_input: usize = 0;
    let mut line_buffer = [0u8; BYTES_PER_LINE];

    while let Ok(bytes_read) = buffer.read(&mut line_buffer) {
        if bytes_read == 0 {
            break;
        }

        // Print the offset
        print!("[0x{:08x}] ", position_in_input);

        // Print the hex representation
        for byte in &line_buffer[..bytes_read] {
            print!("{:02x} ", byte);
        }

        // Add padding for incomplete lines
        if bytes_read < BYTES_PER_LINE {
            for _ in 0..(BYTES_PER_LINE - bytes_read) {
                print!("   ");
            }
        }

        // Print ASCII representation
        print!("| ");
        for byte in &line_buffer[..bytes_read] {
            if byte.is_ascii_graphic() || *byte == b' ' {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }

        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: Could not open file '{}': {}", file_path.display(), e);
            std::process::exit(1);
        }
    };

    print_hexdump(file)
}
