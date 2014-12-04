use std::io::File;
use std::os;
use std::fmt;

#[repr(C)]
struct Complex {
    re: f32,
    im: f32,
}

#[link(name = "m")]
extern {
    fn csqrtf(z: Complex) -> Complex;
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("usage: read_blend_file /path/to/file.blend");
    } else {
        let slice: &str = args[1].as_slice();
        println!("open \"{}\" ...", slice);
        {
            // read 7 bytes from the Blender file
            let path = Path::new(slice);
            let display = path.display();
            let mut file = match File::open(&path) {
                Err(why) => fail!("couldn't open {}: {}", display, why.desc),
                Ok(file) => file,
            };
            let mut buf = [0u8, ..7];
            match file.read(buf) {
                Err(why) => fail!("couldn't open {}: {}", display, why.desc),
                Ok(_) => (),
            }
            // pack those 7 bytes into a string ...
            let mut string = String::new();
            for n in range(0u, 7) {
                string.push(buf[n] as char);
            }
            // ... to be able to compare them
            println!("First 7 bytes match \"BLENDER\"? {}", string == String::from_str("BLENDER"));
        } // file gets closed here
        // code below is temporary
        let z = Complex { re: -1.0, im: 0.0 };
        let z_sqrt = unsafe {
            csqrtf(z)
        };
        println!("the square root of {} is {}", z, z_sqrt);
    }
}

impl fmt::Show for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
