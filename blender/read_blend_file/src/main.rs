// use std::io::File;
use std::os;

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("usage: read_blend_file /path/to/file.blend");
    } else {
        println!("open \"{}\" ...", args[1]);
    }
}
