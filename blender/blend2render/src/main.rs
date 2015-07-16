extern crate getopts;
use getopts::Options;
use std::env;
use std::fs::File;
use std::io;

fn read_blend_file(inp: &str) -> io::Result<()> {
    // open file
    let file = try!(File::open(inp));
    // TODO: read 12 bytes from the Blender file
    Ok(())
}

fn do_work(inp: &str, out: Option<String>) {
    println!("FILE = {}", inp);
    match out {
        Some(x) => println!("output file name: {}", x),
        None => println!("no output file name"),
    }
    read_blend_file(inp);
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
