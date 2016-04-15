pub mod lagrange {

    pub fn interpolation(points: &[(f32, f32)], x : f32) -> f32 {
        let mut i : i32 = -1;
        let res = points
                    .iter()
                    .fold(0.0 as f32, |acc, &(xi, yi)| {
                        i+=1;
                        return acc + yi * lsum(points, i, x);
                    });



        return res;
    }

    fn lsum(points: &[(f32, f32)], i: i32, x: f32) -> f32 {
        let (xi, _) = points[i as usize];  // usize overflow?

        let mut upper : Vec<f32> = points
                            .iter()
                            .map( |&(xj, yj)| x - xj ).collect();
        upper[i as usize] = 1.0;
        let upper_res : f32 = upper.into_iter().fold(1.0 as f32, |acc, v| acc * v );

        let mut lower : Vec<f32> = points
                            .iter()
                            .map( |&(xj, yj)| xi - xj ).collect();
        lower[i as usize] = 1.0;
        let lower_res : f32 = lower.into_iter().fold(1.0 as f32, |acc, v| acc * v );

        return upper_res / lower_res;
    }
}
