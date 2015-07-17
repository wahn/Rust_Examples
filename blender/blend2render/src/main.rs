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
    let mut file = File::open(inp).unwrap();
    // read 12 bytes from the Blender file
    let mut buf = [0u8; 12];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...", bytes_read, buf.len());
        return Ok(());
    }
    let mut header = String::new();
    for e in buf.iter() {
        header.push(*e as char);
    }
    println!("header = \"{}\"", header);
    // compare first 7 chars to "BLENDER"
    if header.len() >= 7 {
        // create copy of header without moving value
        let mut blender = String::new();
        blender = blender + &header;
        blender.truncate(7); // first 7 chars
        if blender == "BLENDER" {
            println!("starts with \"BLENDER\" ...");
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
            if ptr_size == 8 && (ptr_size_differs | switch_endian) == false {
                // assumes 64-bit pointers (in file as well as on platform)
                // we also don't handle endian switching (yet)
                return read_remaining_blend_file(file);
            } else {
                println!("TODO: ptr_size = {}", ptr_size);
                println!("TODO: ptr_size_differs = {}", ptr_size_differs);
                println!("TODO: switch_endian = {}", switch_endian);
            }
        } else {
            println!("ERROR: FILE is not a Blender file");
        }
    } else {
        println!("ERROR: FILE is not a Blender file");
    }
    Ok(())
}

fn read_remaining_blend_file(mut file: File) -> io::Result<()> {
    // read_file_dna
    // 4 * int + 64-bit pointer
    let mut buf = [0u8; 24];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...", bytes_read, buf.len());
        return Ok(());
    }
    let mut bhead = String::new();
    for e in buf.iter() {
        bhead.push(*e as char);
    }
    let mut code = String::new();
    code.push_str(&bhead); // copy
    code.truncate(4); // first 4 chars
    println!("code = {}", code);
    // first int is 'len'
    let mut len: u32 = 0;
    len += (buf[4] as u32) <<  0;
    len += (buf[5] as u32) <<  8;
    len += (buf[6] as u32) << 16;
    len += (buf[7] as u32) << 24;
    println!("len = {}", len);
    // TODO: if code == "ENDB" && len == 0 { ... }
    if code == "ENDB" && len == 0 {
        println!("TODO: code == \"{}\"", code);
    } else if code == "DNA1" {
        println!("TODO: code == \"{}\"", code);
    } else {
        let mut tc = String::new();
        tc.push_str(&bhead); // copy
        tc.truncate(2); // first 4 chars
        if tc == "CA" {
            println!("TODO: tc == \"{}\"", tc);
        } else {
            let mut dummy: Vec<u8> = Vec::with_capacity(len as usize);
            for i in 0..len {
                dummy.push(i as u8);
            }
            let mut buf = &mut dummy;
            let bytes_read = file.read(buf).unwrap();
            if bytes_read != buf.len() {
                println!("{} bytes read, but {} expected ...", bytes_read, buf.len());
                return Ok(());
            } else {
                println!("{} bytes read ...", bytes_read);
            }
        }
    }
    // WORK: read remaining file
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
