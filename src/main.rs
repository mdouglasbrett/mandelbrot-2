use std::env;
use std::process::exit;

use mandelbrot_2::{parse_complex, parse_pair, render, write_image};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example, {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];
    println!("Rendering pixels");
    render(&mut pixels, bounds, upper_left, lower_right);
    println!("Writing pixels to file");
    write_image(&args[1], &pixels, bounds).expect("err writing PNG file");
}
