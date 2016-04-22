pub mod lagrange {

    pub fn interpolation(points: &[(f64, f64)], x : f64) -> f64 {
        let mut i : i32 = -1;
        let res = points
                    .iter()
                    .fold(0.0 as f64, |acc, &(xi, yi)| {
                        i+=1;
                        return acc + yi * lsum(points, i, x);
                    });



        return res;
    }

    fn lsum(points: &[(f64, f64)], i: i32, x: f64) -> f64 {
        let (xi, _) = points[i as usize];  // usize overflow?

        let mut upper : Vec<f64> = points
                            .iter()
                            .map( |&(xj, yj)| x - xj ).collect();
        upper[i as usize] = 1.0;
        let upper_res : f64 = upper.into_iter().fold(1.0 as f64, |acc, v| acc * v );

        let mut lower : Vec<f64> = points
                            .iter()
                            .map( |&(xj, yj)| xi - xj ).collect();
        lower[i as usize] = 1.0;
        let lower_res : f64 = lower.into_iter().fold(1.0 as f64, |acc, v| acc * v );

        return upper_res / lower_res;
    }
}
