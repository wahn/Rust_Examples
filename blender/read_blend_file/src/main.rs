use std::io::File;
use std::mem;
use std::os;

// https://stackoverflow.com/questions/24633784/is-there-a-gzip-library-available-for-rust

fn pointer_size() -> uint {
    let tmp = 0u8;
    let boxed = box tmp;
    mem::size_of_val(&boxed)
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("usage: read_blend_file /path/to/file.blend");
    } else {
        let slice: &str = args[1].as_slice();
        println!("open \"{}\" ...", slice);
        {
            // read 12 bytes from the Blender file
            let path = Path::new(slice);
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => fail!("couldn't open {}: {}", display, why.desc),
                Ok(file) => file,
            };
            let mut buf = [0u8, ..12];
            match file.read(buf) {
                Err(why) => println!("{}", why),
                Ok(_) => (),
            }
            // pack those 12 bytes into a string ...
            let mut header = String::new();
            for n in range(0u, 12) {
                header.push(buf[n] as char);
            }
            // ... to be able to compare them
            let slice = header.as_slice();
            let blender: &str = slice.slice(0, 7); // first 7 chars
            if blender == "BLENDER" {
                println!("INFO: a Blender file.");
                let ptr_size = pointer_size();
                println!("size of pointer in bytes: {}", ptr_size);
                println!("Header (12 chars): {}", header);
                let mut ptr_size_differs = false;
                if buf[7] as char == '_' {
                    // 32-bit pointers expected
                    if ptr_size != 4 {
                        ptr_size_differs = true;
                    }
                    println!("32-bit pointers in file, pointer size differs? {}", ptr_size_differs);
                } else {
                    // 64-bit pointers expected
                    if ptr_size != 8 {
                        ptr_size_differs = true;
                    }
                    println!("64-bit pointers in file, pointer size differs? {}", ptr_size_differs);
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
                if buf[8] as char == 'v' {
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
                let mut num = String::new();
                for n in range(9u, 12) {
                    num.push(buf[n] as char);
                }
                println!("num = {}", num);
                // assumes 64-bit pointers (in file as well as on platform)
                loop {
                    // read_file_dna
                    let mut bhead8 = [0u8, ..24]; // 4 * int + 64-bit pointer
                    match file.read(bhead8) {
                        Err(why) => println!("{}", why),
                        Ok(_) => (),
                    }
                    // pack those 24 bytes into a string ...
                    let mut bhead = String::new();
                    for n in range(0u, 24) {
                        bhead.push(bhead8[n] as char);
                    }
                    let slice = bhead.as_slice();
                    let code: &str = slice.slice(0, 4); // first 4 chars
                    println!("code = {}", code);
                    let mut len: uint = 0;
                    len += (bhead8[4] as uint) <<  0;
                    len += (bhead8[5] as uint) <<  8;
                    len += (bhead8[6] as uint) << 16;
                    len += (bhead8[7] as uint) << 24;
                    println!("len = {}", len);
                    if code == "ENDB" && len == 0 {
                        break;
                    } else if code == "DNA1" {
                        // DNA_sdna_from_data
                        let mut counter = 0u;
                        // SDNA
                        let mut char4 = [0u8, ..4];
                        match file.read(char4) {
                            Err(why) => println!("{}", why),
                            Ok(_) => (),
                        }
                        counter += 4;
                        let mut str4 = String::new();
                        for n in range(0u, 4) {
                            str4.push(char4[n] as char);
                        }
                        let slice = str4.as_slice();
                        let code: &str = slice.slice(0, 4);
                        println!("  code = {}", code);
                        if code == "SDNA" {
                            // NAME
                            let mut char4 = [0u8, ..4];
                            match file.read(char4) {
                                Err(why) => println!("{}", why),
                                Ok(_) => (),
                            }
                            counter += 4;
                            let mut str4 = String::new();
                            for n in range(0u, 4) {
                                str4.push(char4[n] as char);
                            }
                            let slice = str4.as_slice();
                            let code: &str = slice.slice(0, 4);
                            println!("  code = {}", code);
                            if code == "NAME" {
                                // nr_names
                                let io_result = file.read_le_u32();
                                let nr_names: u32 = io_result.unwrap();
                                counter += 4;
                                println!("  nr_names = {}", nr_names);
                                let mut nr = 0u32;
                                loop {
                                    let mut name = String::new();
                                    loop {
                                        // expect strings with '\0' as terminator
                                        let io_result = file.read_byte();
                                        let byte: u8 = io_result.unwrap();
                                        counter += 1;
                                        if byte != 0 {
                                            name.push(byte as char);
                                        } else {
                                            println!("    name = '{}'", name);
                                            nr += 1;
                                            break;
                                        }
                                    } // name loop
                                    if nr >= nr_names {
                                        break;
                                    }
                                } // nr_names loop
                                // assume fill bytes with '\0'
                                loop {
                                    let io_result = file.read_byte();
                                    let byte: u8 = io_result.unwrap();
                                    counter += 1;
                                    if byte != 0 {
                                        break;
                                    }
                                }
                            }
                        }
                        // read remaining stuff
                        let _dummy = file.read_exact(len - counter);
                    } else {
                        let _dummy = file.read_exact(len);
                    }
                }
            } else {
                println!("ERROR: not a Blender file.");
            }
        } // file gets closed here
    }
}
