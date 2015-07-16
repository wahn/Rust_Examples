extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem;

fn pointer_size() -> usize {
    let tmp = 0u8;
    let boxed = Box::new(tmp);
    mem::size_of_val(&boxed)
}

fn read_blend_file(inp: &str) -> io::Result<()> {
    // open file
    let file = try!(File::open(inp));
    // read 12 bytes from the Blender file
    let mut take = file.take(12u64);
    let mut header = String::new();
    try!(take.read_to_string(&mut header));
    println!("header = \"{}\"", header);
    // compare first 7 chars to "BLENDER"
    if header.len() >= 7 {
        // create copy of header without moving value
        let mut blender = String::new();
        blender = blender + &header;
        blender.truncate(7); // first 7 chars
        if blender == "BLENDER" {
            println!("starts with \"BLENDER\" ...");
            // TODO: check version (next 5 chars in header)

            // switch to byte copy of String representation
            let bytes = header.into_bytes();
            // check 8th char of header
            println!("8th char of header = '{}'", bytes[7] as char);
            // check for 32-bit pointers vs. 64-bit pointers
            let ptr_size = pointer_size();
            println!("size of pointer in bytes: {}", ptr_size);
            let mut ptr_size_differs = false;
            if bytes[7] as char == '_' {
                // 32-bit pointers expected
                if ptr_size != 4 {
                    ptr_size_differs = true;
                }
                println!("32-bit pointers in file, pointer size differs? {}",
                         ptr_size_differs);
            } else {
                // 64-bit pointers expected
                if ptr_size != 8 {
                    ptr_size_differs = true;
                }
                println!("64-bit pointers in file, pointer size differs? {}",
                         ptr_size_differs);
            }
            // little endian or big endian?
            let mut l_endian;
            let endian_test = 1u16; // value one stored in two bytes
            if endian_test.to_le() != endian_test {
                l_endian = false;
            } else {
                // if we are on a little endian machine to_le() is a no-op
                l_endian = true;
            }
            println!("l_endian = {}", l_endian);
            let mut switch_endian;
            if bytes[8] as char == 'v' {
                // file stores little endian
                if !l_endian {
                    switch_endian = true;
                } else {
                    switch_endian = false;
                }
            } else {
                // file stores big endian
                if l_endian {
                    switch_endian = true;
                } else {
                    switch_endian = false;
                }
            }
            println!("switch_endian = {}", switch_endian);
            // get the version number
            let last3c = vec!(bytes[9], bytes[10], bytes[11]);
            let version = String::from_utf8(last3c).unwrap(); // last 3 chars
            println!("version = {}", version);
            // WORK: read remaining file
        } else {
            println!("ERROR: FILE is not a Blender file");
        }
    } else {
        println!("ERROR: FILE is not a Blender file");
    }
    Ok(())
}

fn do_work(inp: &str, out: Option<String>) {
    println!("FILE = {}", inp);
    match out {
        Some(x) => println!("output file name: {}", x),
        None => println!("no output file name"),
    }
    match read_blend_file(inp) {
        Ok(_) => { ; }
        Err(f) => { panic!(f.to_string()) }
    };
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    do_work(&input, output);
}
