use std::io::File;
use std::mem;
use std::num::Int;
use std::os;

// https://stackoverflow.com/questions/24633784/is-there-a-gzip-library-available-for-rust

fn pointer_size() -> usize {
    let tmp = 0u8;
    let boxed = Box::new(tmp);
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
                Err(why) => panic!("couldn't open {}: {}", display, why.desc),
                Ok(file) => file,
            };
            let io_result = file.read_exact(12);
            let buf = io_result.unwrap();
            // pack those 12 bytes into a string ...
            let mut header = String::new();
            for e in buf.iter() {
                header.push(*e as char);
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
                let num: &str = slice.slice(9, 12); // last 3 chars
                println!("num = {}", num);
                // assumes 64-bit pointers (in file as well as on platform)
                loop {
                    // read_file_dna
                    let io_result = file.read_exact(24); // 4 * int + 64-bit pointer
                    let bhead8 = io_result.unwrap();
                    // pack those 24 bytes into a string ...
                    let mut bhead = String::new();
                    for e in bhead8.iter() {
                        bhead.push(*e as char);
                    }
                    let slice = bhead.as_slice();
                    let code: &str = slice.slice(0, 4); // first 4 chars
                    println!("code = {}", code);
                    let mut len: u32 = 0;
                    len += (bhead8[4] as u32) <<  0;
                    len += (bhead8[5] as u32) <<  8;
                    len += (bhead8[6] as u32) << 16;
                    len += (bhead8[7] as u32) << 24;
                    println!("len = {}", len);
                    if code == "ENDB" && len == 0 {
                        break;
                    } else if code == "DNA1" {
                        // DNA_sdna_from_data
                        let mut counter = 0u32;
                        // SDNA
                        let io_result = file.read_exact(4);
                        let char4 = io_result.unwrap();
                        counter += 4;
                        let mut str4 = String::new();
                        for e in char4.iter() {
                            str4.push(*e as char);
                        }
                        let slice = str4.as_slice();
                        let code: &str = slice.slice(0, 4);
                        println!("  code = {}", code);
                        if code == "SDNA" {
                            // NAME
                            let io_result = file.read_exact(4);
                            let char4 = io_result.unwrap();
                            counter += 4;
                            let mut str4 = String::new();
                            for e in char4.iter() {
                                str4.push(*e as char);
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
                                let mut last_byte;
                                loop {
                                    let io_result = file.read_byte();
                                    let byte: u8 = io_result.unwrap();
                                    counter += 1;
                                    if byte != 0 {
                                        last_byte = byte;
                                        break;
                                    }
                                } // fill bytes
                                // TYPE
                                let io_result = file.read_exact(3);
                                let char3 = io_result.unwrap();
                                counter += 3;
                                let mut str4 = String::new();
                                str4.push(last_byte as char);
                                for e in char3.iter() {
                                    str4.push(*e as char);
                                }
                                let slice = str4.as_slice();
                                let code: &str = slice.slice(0, 4);
                                println!("  code = {}", code);
                                if code == "TYPE" {
                                    // nr_types
                                    let io_result = file.read_le_u32();
                                    let nr_types: u32 = io_result.unwrap();
                                    counter += 4;
                                    println!("  nr_types = {}", nr_types);
                                    let mut nr = 0u32;
                                    loop {
                                        let mut name = String::new();
                                        loop {
                                            // expect strings with
                                            // '\0' as terminator
                                            let io_result = file.read_byte();
                                            let byte: u8 = io_result.unwrap();
                                            counter += 1;
                                            if byte != 0 {
                                                name.push(byte as char);
                                            } else {
                                                println!("    type = '{}'",
                                                         name);
                                                nr += 1;
                                                break;
                                            }
                                        } // type loop
                                        if nr >= nr_types {
                                            break;
                                        }
                                    } // nr_types loop
                                    // assume fill bytes with '\0'
                                    let mut last_byte;
                                    loop {
                                        let io_result = file.read_byte();
                                        let byte: u8 = io_result.unwrap();
                                        counter += 1;
                                        if byte != 0 {
                                            last_byte = byte;
                                            break;
                                        }
                                    } // fill bytes
                                    // TLEN
                                    let io_result = file.read_exact(3);
                                    let char3 = io_result.unwrap();
                                    counter += 3;
                                    let mut str4 = String::new();
                                    str4.push(last_byte as char);
                                    for e in char3.iter() {
                                        str4.push(*e as char);
                                    }
                                    let slice = str4.as_slice();
                                    let code: &str = slice.slice(0, 4);
                                    println!("  code = {}", code);
                                    if code == "TLEN" {
                                        // typelens
                                        let io_result = file.read_le_u16();
                                        let typelens: u16 = io_result.unwrap();
                                        counter += 2;
                                        println!("    typelens = {}", typelens);
                                        // skip nr_types times u16 values
                                        let _dummy = file.read_exact((2u32 *
                                                                     nr_types) as
                                                                     usize);
                                        counter += 2u32 * nr_types as u32;
                                        // check next byte
                                        let io_result = file.read_byte();
                                        last_byte = io_result.unwrap();
                                        counter += 1;
                                        if last_byte == 0 {
                                            let io_result = file.read_byte();
                                            last_byte = io_result.unwrap();
                                            counter += 1;
                                        }
                                        // STRC
                                        let io_result = file.read_exact(3);
                                        let char3 = io_result.unwrap();
                                        counter += 3;
                                        let mut str4 = String::new();
                                        str4.push(last_byte as char);
                                        for e in char3.iter() {
                                            str4.push(*e as char);
                                        }
                                        let slice = str4.as_slice();
                                        let code: &str = slice.slice(0, 4);
                                        println!("  code = {}", code);
                                        if code == "STRC" {
                                            // nr_structs
                                            let io_result = file.read_le_u32();
                                            let nr_structs: u32 = io_result.unwrap();
                                            counter += 4;
                                            println!("    nr_structs = {}", nr_structs);
                                            nr = 0u32;
                                            loop {
                                                // sp0
                                                let io_result = file.read_le_u16();
                                                let _sp0: u16 = io_result.unwrap();
                                                counter += 2;
                                                // sp1
                                                let io_result = file.read_le_u16();
                                                let sp1: u16 = io_result.unwrap();
                                                counter += 2;
                                                for _n in range(0u16, 2 * sp1) {
                                                    let io_result = file.read_le_u16();
                                                    let _u: u16 = io_result.unwrap();
                                                    counter += 2;
                                                }
                                                nr += 1;
                                                if nr >= nr_structs {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // read remaining stuff
                        println!("skip {} bytes", len - counter);
                        let _dummy = file.read_exact((len - counter) as usize);
                    } else {
                        let tc: &str = slice.slice(0, 2); // first 2 chars
                        if tc == "CA" {
                            let mut counter = 0u32;
                            // struct ID (see DNA_ID.h)
                            let _next = file.read_le_u64();
                            counter += 8;
                            let _prev = file.read_le_u64();
                            counter += 8;
                            let _newid = file.read_le_u64();
                            counter += 8;
                            let _lib = file.read_le_u64();
                            counter += 8;
                            let io_result = file.read_exact(66);
                            let bname = io_result.unwrap();
                            // pack those 66 bytes into a string ...
                            let mut name = String::new();
                            for e in bname.iter() {
                                if *e == 0u8 {
                                    // ... but stop as soon as you see '\0'
                                    break;
                                } else {
                                    name.push(*e as char);
                                }
                            }
                            counter += 66;
                            let _flag = file.read_le_u16();
                            counter += 2;
                            let _us = file.read_le_u32();
                            counter += 4;
                            let _icon_id = file.read_le_u32();
                            counter += 4;
                            let _pad2 = file.read_le_u32();
                            counter += 4;
                            let _properties = file.read_le_u64();
                            counter += 8;
                            // struct Camera (see DNA_camera_types.h)
                            let _adt = file.read_le_u64();
                            counter += 8;
                            let io_result = file.read_u8();
                            let cam_type: u8 = io_result.unwrap();
                            counter += 1;
                            let _dtx = file.read_u8();
                            counter += 1;
                            let _flag = file.read_le_u16();
                            counter += 2;
                            let _passepartalpha = file.read_le_f32();
                            counter += 4;
                            let _clipsta = file.read_le_f32();
                            counter += 4;
                            let _clipend = file.read_le_f32();
                            counter += 4;
                            let io_result = file.read_le_f32();
                            let lens: f32 = io_result.unwrap();
                            counter += 4;
                            let _dummy = file.read_exact((len - counter) as usize);
                            if cam_type == 0u8 {
                                println!("Camera({}, {}, {})", name, "CAM_PERSP", lens)
                            } else {
                                println!("Camera({}, {}, {})", name, cam_type, lens)
                            }
                        } else {
                            let _dummy = file.read_exact(len as usize);
                        }
                    }
                }
            } else {
                println!("ERROR: not a Blender file.");
            }
        } // file gets closed here
    }
}
