use num::Complex;

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit' of
/// iterations it took to decide.
///
/// If 'c' is not a member, return Some(i), where 'i' is the number of iterations
/// it took for 'c' to leave the circle of radius 2 centered on the origin. If 'c'
/// seems to be a member (more precisely, if we reached the iteration limit without
/// being able to prove 'c' is a not a member), return None.
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    None
}
