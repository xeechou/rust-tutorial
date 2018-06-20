extern crate num;
use num::Complex;

extern crate image;
use image::ColorType;
use image::png::PNGEncoder;

use std::fs::File;
use std::str::FromStr;

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex {re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}


/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations.
///
/// If `c` is not a member, return `Some(i)`. where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be member return `None`.
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex {re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    return None;
}

//so the power of match is that we can have some information back in the Some,
// for example, find returns a offset.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            //okay, this line is really awesome, basically we canse uses
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}


fn parse_complex(s: &str) -> Option<Complex<f64>>
{
    //I guess you can use
    let res : Option<Complex<f64>> =
        match parse_pair(s, ',') {
            Some((re, im)) => Some(Complex {re, im}),
            None => None
        };
    return res;
}


#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",         ','), None);
    assert_eq!(parse_pair::<i32>("10,",      ','), None);
    assert_eq!(parse_pair::<i32>(",10",      ','), None);
    assert_eq!(parse_pair::<i32>("10,10",    ','), Some((10,10)));
    assert_eq!(parse_pair::<i32>("10,10,10", ','), None);
}

// 0,1 to
fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
                  upper_left: Complex<f64>, lower_right: Complex<f64>)
                  -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    return Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    };
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im: 1.0},
                              Complex {re: 1.0, im: -1.0}),
    Complex {re: -0.5, im: -0.5});
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0 .. bounds.1 {
        for col in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (col, row),
                                       upper_left, lower_right);
            pixels[row * bounds.0 + col] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}


fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;
    Ok(())

}

use std::io::Write;
extern crate crossbeam;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 - 1.20,0.35 -1,20",
                 args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left cornor point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right cornor point");

    //println!("bounds: {:?}, upper_left: {:?}, lower_right: {:?}", bounds, upper_left, lower_right);
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    let mut pixels = vec![0; bounds.0 * bounds.1];
    {
        let bands :Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
                                                      upper_left, lower_right);
                spawner.spawn(move || {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
//    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}
