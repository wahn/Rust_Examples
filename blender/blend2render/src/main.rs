#![allow(non_snake_case)]

extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    loop {
        // read_file_dna
        // 4 * int + 64-bit pointer
        let mut buf = [0u8; 24];
        let bytes_read = file.read(&mut buf).unwrap();
        if bytes_read != buf.len() {
            println!("{} bytes read, but {} expected ...",
                     bytes_read, buf.len());
            return Ok(());
        }
        let mut bhead = String::new();
        for e in buf.iter() {
            bhead.push(*e as char);
        }
        let mut code = String::new();
        code.push_str(&bhead); // copy
        code.truncate(4); // first 4 chars
        //println!("code = {}", code);
        // first int is 'len'
        let mut len: u32 = 0;
        len += (buf[4] as u32) <<  0;
        len += (buf[5] as u32) <<  8;
        len += (buf[6] as u32) << 16;
        len += (buf[7] as u32) << 24;
        //println!("len = {}", len);
        // TODO: if code == "ENDB" && len == 0 { ... }
        if code == "ENDB" && len == 0 {
            println!("TODO: code == \"{}\"", code);
            break;
        } else if code == "DNA1" {
            println!("TODO: code == \"{}\"", code);
            break;
        } else {
            let mut tc = String::new();
            tc.push_str(&bhead); // copy
            tc.truncate(2); // first 4 chars
            if tc == "CA" {
                let mut counter = 0u32;
                let mut name = String::new();
                match read_struct_id(&file, &mut counter, &mut name) {
                    Ok(_) => { ; }
                    Err(f) => { panic!(f.to_string()) }
                };
                println!("name = {}", name);
                // struct Camera (see DNA_camera_types.h)
                // adt
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // cam_type
                let mut buf = [0u8; 1];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 1;
                let cam_type: u8 = buf[0];
                // dtx
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 1;
                // flag
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                // passepartalpha
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // clipsta
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // clipend
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // lens
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let lens: f32 = unsafe { mem::transmute(buf) };
                //println!("{} bytes read ...", counter);
                // read remaining bytes, but don't use them (yet)
                let mut dummy: Vec<u8> = Vec::with_capacity((len - counter)
                                                            as usize);
                for i in 0..(len - counter) {
                    dummy.push(i as u8);
                }
                let mut buf = &mut dummy;
                let bytes_read = file.read(buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                } else {
                    //println!("{} bytes read ...", bytes_read);
                }
                if cam_type == 0u8 {
                    println!("Camera({}, {}, {})", name, "CAM_PERSP", lens)
                } else {
                    println!("Camera({}, {}, {})", name, cam_type, lens)
                }
            } else if tc == "OB" {
                let mut counter = 0u32;
                let mut name = String::new();
                match read_struct_id(&file, &mut counter, &mut name) {
                    Ok(_) => { ; }
                    Err(f) => { panic!(f.to_string()) }
                };
                println!("name = {}", name);
                // struct Object (see DNA_object_types.h)
                // adt
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // sculpt
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // type
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                // partype
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                // par1
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // par2
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // par3
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // parsubstr (64 bytes)
                let mut buf = [0u8; 64];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 64;
                // pack those 64 bytes into a string ...
                let mut parsubstr = String::new();
                for e in buf.iter() {
                    if *e == 0u8 {
                        // ... but stop as soon as you see '\0'
                        break;
                    } else {
                        parsubstr.push(*e as char);
                    }
                }
                //println!("parsubstr = \"{}\"", parsubstr);
                // parent
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // track
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // proxy
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // proxy_group
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // proxy_from
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // ipo
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // bb
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // action
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // poselib
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // pose
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // data
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // gpd
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // avs (see bAnimVizSettings in DNA_action_types.h)
                // 4*4 + 8*2 + 4*4 = 48 bytes
                let mut buf = [0u8; 48];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 48;
                // mpath
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // 4 ListBase entries (2 pointers each - DNA_listBase.h)
                // 8*2*4 = 64 bytes
                let mut buf = [0u8; 64];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 64;
                // mode
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // restore_mode
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // mat
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // matbits
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // totcol
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // actcol
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // loc[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("loc[3] = ({}, {}, {})", x, y, z);
                // dloc[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("dloc[3] = ({}, {}, {})", x, y, z);
                // orig[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("orig[3] = ({}, {}, {})", x, y, z);
                // size[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("size[3] = ({}, {}, {})", x, y, z);
                // dsize[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("dsize[3] = ({}, {}, {})", x, y, z);
                // dscale[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("dscale[3] = ({}, {}, {})", x, y, z);
                // rot[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("rot[3] = ({}, {}, {})", x, y, z);
                // drot[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("drot[3] = ({}, {}, {})", x, y, z);
                // quat[4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("quat[4] = ({}, {}, {}, {})", a, b, c, d);
                // dquat[4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("dquat[4] = ({}, {}, {}, {})", a, b, c, d);
                // rotAxis[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("rotAxis[3] = ({}, {}, {})", x, y, z);
                // drotAxis[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                let x: f32 = unsafe { mem::transmute(xbuf) };
                let y: f32 = unsafe { mem::transmute(ybuf) };
                let z: f32 = unsafe { mem::transmute(zbuf) };
                println!("drotAxis[3] = ({}, {}, {})", x, y, z);
                // rotAngle
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let rotAngle: f32 = unsafe { mem::transmute(buf) };
                println!("rotAngle = {}", rotAngle);
                // drotAngle
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let drotAngle: f32 = unsafe { mem::transmute(buf) };
                println!("drotAngle = {}", drotAngle);
                // obmat[4][4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("obmat[4][4] =\n    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                // parentinv[4][4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("parentinv[4][4] =\n    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                // constinv[4][4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("constinv[4][4] =\n    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                // imat[4][4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("imat[4][4] =\n    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                // imat_ren[4][4]
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("imat_ren[4][4] =\n    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                let mut abuf = [0u8; 4];
                let bytes_read = file.read(&mut abuf).unwrap();
                if bytes_read != abuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, abuf.len());
                    return Ok(());
                }
                let mut bbuf = [0u8; 4];
                let bytes_read = file.read(&mut bbuf).unwrap();
                if bytes_read != bbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, bbuf.len());
                    return Ok(());
                }
                let mut cbuf = [0u8; 4];
                let bytes_read = file.read(&mut cbuf).unwrap();
                if bytes_read != cbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, cbuf.len());
                    return Ok(());
                }
                let mut dbuf = [0u8; 4];
                let bytes_read = file.read(&mut dbuf).unwrap();
                if bytes_read != dbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, dbuf.len());
                    return Ok(());
                }
                counter += 16;
                let a: f32 = unsafe { mem::transmute(abuf) };
                let b: f32 = unsafe { mem::transmute(bbuf) };
                let c: f32 = unsafe { mem::transmute(cbuf) };
                let d: f32 = unsafe { mem::transmute(dbuf) };
                println!("    ({}, {}, {}, {})", a, b, c, d);
                // lay
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let lay: u32 = unsafe { mem::transmute(buf) };
                println!("lay = {}", lay);
                // ignore remaining bytes (for now)
                println!("len - counter = {}", len - counter);
                // read remaining bytes, but don't use them (yet)
                let mut dummy: Vec<u8> = Vec::with_capacity((len - counter)
                                                            as usize);
                for i in 0..(len - counter) {
                    dummy.push(i as u8);
                }
                let mut buf = &mut dummy;
                let bytes_read = file.read(buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                } else {
                    //println!("{} bytes read ...", bytes_read);
                }
            } else if tc == "MA" {
                let mut counter = 0u32;
                let mut name = String::new();
                match read_struct_id(&file, &mut counter, &mut name) {
                    Ok(_) => { ; }
                    Err(f) => { panic!(f.to_string()) }
                };
                println!("name = {}", name);
                // struct Material (see DNA_material_types.h)
                // adt
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // material_type
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                let material_type: i16 = unsafe { mem::transmute(buf) };
                match material_type {
                    0 => { println!("material_type = MA_TYPE_SURFACE"); }
                    1 => { println!("material_type = MA_TYPE_HALO"); }
                    2 => { println!("material_type = MA_TYPE_VOLUME"); }
                    3 => { println!("material_type = MA_TYPE_WIRE"); }
                    _ => { panic!("material_type = {}", material_type); }
                };
                // flag
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                // r
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let r: f32 = unsafe { mem::transmute(buf) };
                println!("r = {}", r);
                // g
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let g: f32 = unsafe { mem::transmute(buf) };
                println!("g = {}", g);
                // b
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let b: f32 = unsafe { mem::transmute(buf) };
                println!("b = {}", b);
                // specr
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let specr: f32 = unsafe { mem::transmute(buf) };
                println!("specr = {}", specr);
                // specg
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let specg: f32 = unsafe { mem::transmute(buf) };
                println!("specg = {}", specg);
                // specb
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let specb: f32 = unsafe { mem::transmute(buf) };
                println!("specb = {}", specb);
                // mirr
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let mirr: f32 = unsafe { mem::transmute(buf) };
                println!("mirr = {}", mirr);
                // mirg
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let mirg: f32 = unsafe { mem::transmute(buf) };
                println!("mirg = {}", mirg);
                // mirb
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let mirb: f32 = unsafe { mem::transmute(buf) };
                println!("mirb = {}", mirb);
                // ambr
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let ambr: f32 = unsafe { mem::transmute(buf) };
                println!("ambr = {}", ambr);
                // ambg
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let ambg: f32 = unsafe { mem::transmute(buf) };
                println!("ambg = {}", ambg);
                // ambb
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let ambb: f32 = unsafe { mem::transmute(buf) };
                println!("ambb = {}", ambb);
                // amb
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let amb: f32 = unsafe { mem::transmute(buf) };
                println!("amb = {}", amb);
                // emit
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let emit: f32 = unsafe { mem::transmute(buf) };
                println!("emit = {}", emit);
                // ang
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let ang: f32 = unsafe { mem::transmute(buf) };
                println!("ang = {}", ang);
                // spectra
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let spectra: f32 = unsafe { mem::transmute(buf) };
                println!("spectra = {}", spectra);
                // ray_mirror
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let ray_mirror: f32 = unsafe { mem::transmute(buf) };
                println!("ray_mirror = {}", ray_mirror);
                // alpha
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let alpha: f32 = unsafe { mem::transmute(buf) };
                println!("alpha = {}", alpha);
                // ref (is a keyword)
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let refl: f32 = unsafe { mem::transmute(buf) };
                println!("ref = {}", refl);
                // spec
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let spec: f32 = unsafe { mem::transmute(buf) };
                println!("spec = {}", spec);
                // zoffs
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let zoffs: f32 = unsafe { mem::transmute(buf) };
                println!("zoffs = {}", zoffs);
                // add
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let add: f32 = unsafe { mem::transmute(buf) };
                println!("add = {}", add);
                // translucency
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let translucency: f32 = unsafe { mem::transmute(buf) };
                println!("translucency = {}", translucency);
                // struct VolumeSettings (see DNA_material_types.h)
                // 4*4 + 3*3*4 + 3*4 + 4*2 + 4*4 = 88 bytes
                let mut buf = [0u8; 88];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 88;
                // struct GameSettings (see DNA_material_types.h)
                // 4*4 = 16 bytes
                let mut buf = [0u8; 16];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 16;
                // fresnel_mir - seed2
                // 7*4 + 3*2 + 2*1 = 36 bytes
                let mut buf = [0u8; 36];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 36;
                // gloss_mir - shade_flag
                // 2*4 + 2*2 + 4*4 + 2*2 = 32 bytes
                let mut buf = [0u8; 32];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 32;
                // mode - strand_uvname
                // 4*4 + 4*2 + 10*4 = 128 bytes
                let mut buf = [0u8; 128];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 128;
                // sbias - septex
                // 3*4 + 4 = 16 bytes
                let mut buf = [0u8; 16];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 16;
                // rgbsel - pad
                // 4*1 + 3*2 + 2*1 = 12 bytes
                let mut buf = [0u8; 12];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 12;
                // diff_shader
                let mut buf = [0u8; 2];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                let diff_shader: i16 = unsafe { mem::transmute(buf) };
                match diff_shader {
                    0 => { println!("diff_shader = MA_DIFF_LAMBERT"); }
                    1 => { println!("diff_shader = MA_DIFF_ORENNAYAR"); }
                    2 => { println!("diff_shader = MA_DIFF_TOON"); }
                    3 => { println!("diff_shader = MA_DIFF_MINNAERT"); }
                    4 => { println!("diff_shader = MA_DIFF_FRESNEL"); }
                    _ => { panic!("diff_shader = {}", diff_shader); }
                };
                // spec_shader
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 2;
                let spec_shader: i16 = unsafe { mem::transmute(buf) };
                match spec_shader {
                    0 => { println!("spec_shader = MA_SPEC_COOKTORR"); }
                    1 => { println!("spec_shader = MA_SPEC_PHONG"); }
                    2 => { println!("spec_shader = MA_SPEC_BLINN"); }
                    3 => { println!("spec_shader = MA_SPEC_TOON"); }
                    4 => { println!("spec_shader = MA_SPEC_WARDISO"); }
                    _ => { panic!("spec_shader = {}", spec_shader); }
                };
                // ignore remaining bytes (for now)
                println!("len - counter = {}", len - counter);
                // read remaining bytes, but don't use them (yet)
                let mut dummy: Vec<u8> = Vec::with_capacity((len - counter)
                                                            as usize);
                for i in 0..(len - counter) {
                    dummy.push(i as u8);
                }
                let mut buf = &mut dummy;
                let bytes_read = file.read(buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                } else {
                    //println!("{} bytes read ...", bytes_read);
                }
            } else if tc == "ME" {
                let mut counter = 0u32;
                let mut name = String::new();
                match read_struct_id(&file, &mut counter, &mut name) {
                    Ok(_) => { ; }
                    Err(f) => { panic!(f.to_string()) }
                };
                println!("name = {}", name);
                // struct Mesh (see DNA_mesh_types.h)
                // adt
                let mut buf = [0u8; 8];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // bb
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // ipo
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // key
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mat
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mselect
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mpoly
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mloop
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mloopuv
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mloopcol
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mface
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mtface
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // tface
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mvert
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // medge
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // dvert
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // mcol
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // texcomesh
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // edit_btmesh
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 8;
                // struct CustomData (see DNA_customdata_types.h)
                // 8 + 42 * 4 + 4*4 + 2*8 = 208
                // vdata
                let mut buf = [0u8; 208];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 208;
                // edata
                let mut buf = [0u8; 208];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 208;
                // fdata
                let mut buf = [0u8; 208];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 208;
                // pdata
                let mut buf = [0u8; 208];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 208;
                // ldata
                let mut buf = [0u8; 208];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 208;

                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totvert: u32 = unsafe { mem::transmute(buf) };
                println!("totvert = {}", totvert);
                // totedge
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totedge: u32 = unsafe { mem::transmute(buf) };
                println!("totedge = {}", totedge);
                // totface
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totface: u32 = unsafe { mem::transmute(buf) };
                println!("totface = {}", totface);
                // totselect
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totselect: u32 = unsafe { mem::transmute(buf) };
                println!("totselect = {}", totselect);

                // totpoly
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totpoly: u32 = unsafe { mem::transmute(buf) };
                println!("totpoly = {}", totpoly);
                // totloop
                let mut buf = [0u8; 4];
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                let totloop: u32 = unsafe { mem::transmute(buf) };
                println!("totloop = {}", totloop);
                // act_face
                let bytes_read = file.read(&mut buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                }
                counter += 4;
                // loc[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                // size[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                // rot[3]
                let mut xbuf = [0u8; 4];
                let bytes_read = file.read(&mut xbuf).unwrap();
                if bytes_read != xbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, xbuf.len());
                    return Ok(());
                }
                let mut ybuf = [0u8; 4];
                let bytes_read = file.read(&mut ybuf).unwrap();
                if bytes_read != ybuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, ybuf.len());
                    return Ok(());
                }
                let mut zbuf = [0u8; 4];
                let bytes_read = file.read(&mut zbuf).unwrap();
                if bytes_read != zbuf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, zbuf.len());
                    return Ok(());
                }
                counter += 12;
                // WORK
                // ignore remaining bytes (for now)
                println!("len - counter = {}", len - counter);
                // read remaining bytes, but don't use them (yet)
                let mut dummy: Vec<u8> = Vec::with_capacity((len - counter)
                                                            as usize);
                for i in 0..(len - counter) {
                    dummy.push(i as u8);
                }
                let mut buf = &mut dummy;
                let bytes_read = file.read(buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                } else {
                    //println!("{} bytes read ...", bytes_read);
                }
            } else {
                let mut dummy: Vec<u8> = Vec::with_capacity(len as usize);
                for i in 0..len {
                    dummy.push(i as u8);
                }
                let mut buf = &mut dummy;
                let bytes_read = file.read(buf).unwrap();
                if bytes_read != buf.len() {
                    println!("{} bytes read, but {} expected ...",
                             bytes_read, buf.len());
                    return Ok(());
                } else {
                    //println!("{} bytes read ...", bytes_read);
                }
            }
        }
    }
    Ok(())
}

fn read_struct_id(mut file: &File,
                  // return values below
                  counter: &mut u32,
                  name: &mut String) -> io::Result<()> {
    // struct ID (see DNA_ID.h)
    let mut buf = [0u8; 8];
    // next
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 8;
    // prev
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 8;
    // newid
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 8;
    // lib
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 8;
    // read 66 bytes
    let mut buf = [0u8; 66];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 66;
    // pack those 66 bytes into a string ...
    for e in buf.iter() {
        if *e == 0u8 {
            // ... but stop as soon as you see '\0'
            break;
        } else {
            (*name).push(*e as char);
        }
    }
    // flag
    let mut buf = [0u8; 2];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 2;
    // us
    let mut buf = [0u8; 4];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 4;
    // icon_id
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 4;
    // pad2
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 4;
    // properties
    let mut buf = [0u8; 8];
    let bytes_read = file.read(&mut buf).unwrap();
    if bytes_read != buf.len() {
        println!("{} bytes read, but {} expected ...",
                 bytes_read, buf.len());
        return Ok(());
    }
    *counter += 8;
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

fn print_version(program: &str) {
    println!("{} {}", program, VERSION);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version number");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("v") {
        print_version(&program);
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
