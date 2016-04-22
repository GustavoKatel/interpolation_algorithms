extern crate lagrange;
extern crate rand;

use rand::distributions::{IndependentSample, Range};

fn func1(x: f64) -> f64 {
    return (x.powi(4) / 11.0).sqrt() - 2.0 * x.powi(3) + x.powi(2) - 37.0;
}

fn main() {

    let points : [(f64, f64); 11] = [
        (0.0, -37.0),
        (1.0, -37.6984886554),
        (2.0, -47.7939546217),
        (3.0, -79.2863978988),
        (4.0, -144.175818487),
        (5.0, -254.462216386),
        (6.0, -422.145591595),
        (7.0, -659.225944116),
        (8.0, -977.703273947),
        (9.0, -1389.57758109),
        (10.0, -1906.84886554)
    ];

    let (x0, _) : (f64, _) = points[0];
    let (xn, _) : (f64, _) = points[points.len()-1];

    let between = Range::new(x0, xn);
    let mut rng = rand::thread_rng();

    let x : f64 = between.ind_sample(&mut rng);

    let px = lagrange::lagrange::interpolation(&points, x);

    let fx = func1(x);

    let epsilon = (fx - px).abs();

    println!("x = {:?}, P(x) = {:?}, f(x) = {:?}, ξ = {:?}", x, px, fx, epsilon);

}
