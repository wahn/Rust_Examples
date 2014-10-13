// use std::io::File;
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
        println!("open \"{}\" ...", args[1]);
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
