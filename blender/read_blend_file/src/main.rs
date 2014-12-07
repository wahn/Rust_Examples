extern crate libc;

use libc::types;
use std::io::File;
use std::fmt;
use std::mem;
use std::os;

#[link(name = "z")]
extern {
    fn gzopen(path: *const types::os::arch::c95::c_char,
              mode: *const types::os::arch::c95::c_char)
              -> types::common::c95::c_void;
}

#[link(name = "z")]
extern {
    fn gzread(file: types::common::c95::c_void,
              buf:  types::common::c95::c_void,
              len:  uint)
              -> int;
}

#[link(name = "z")]
extern {
    fn gzclose(file: types::common::c95::c_void) -> int;
}

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
                Err(why) => fail!("couldn't open {}: {}", display, why.desc),
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
            } else {
                println!("ERROR: not a Blender file.");
            }
        } // file gets closed here
    }
}
