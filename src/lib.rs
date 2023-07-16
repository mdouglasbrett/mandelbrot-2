use num::complex::Complex64;
use std::str::FromStr;

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` of
/// iterations it took to decide.
///
/// If `c` is not a member, return Some(i), where 'i' is the number of iterations
/// it took for `c` to leave the circle of radius 2 centered on the origin. If `c`
/// seems to be a member (more precisely, if we reached the iteration limit without
/// being able to prove `c` is a not a member), return None.
pub fn escape_time(c: Complex64, limit: usize) -> Option<usize> {
    let mut z = Complex64 { re: 0., im: 0. };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Parse the string `s` as a coordinate pair, eg: `"400x600"` or `"1.0,0.5"`
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    // This is sort of gorgeous...
    match s.find(separator) {
        None => None,
        Some(i) => match (T::from_str(&s[..i]), T::from_str(&s[i + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
fn parse_complex(s: &str) -> Option<Complex64> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex64 { re, im }),
        _ => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex64 {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

fn pixel_to_point() {}
