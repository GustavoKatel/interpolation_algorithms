extern crate lagrange;

fn main() {

    let points : [(f32, f32); 3] = [(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];

    let x : f32 = 1.5;

    let y = lagrange::lagrange::interpolation(&points, x);

    println!("{:?}", y);

}
