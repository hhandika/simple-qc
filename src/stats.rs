//! Heru Handika
//! 1 January 2020
//! Module for statistics

#[inline]
fn sort_vector(vec: &[u32]) -> Vec<u32> {
    let mut sorted_vec = vec.to_vec();
    sorted_vec.sort_unstable();

    sorted_vec
}

pub fn median(vec: &[u32]) -> f64 {
    let sorted_vec = sort_vector(&vec);
    let n = sorted_vec.len();
    let midpoint = n / 2;

    let med;
    if n % 2 == 0  {
        med = (sorted_vec [midpoint - 1]  + sorted_vec[midpoint])  as f64 / 2.0
    } else {
        med = sorted_vec[midpoint] as f64
    }
    
    med 
}

#[inline(always)]
fn sum_of_square(vec: &[f64]) -> f64 {
    let d: f64 = vec.iter()
                    .map(|val| val.powf(2.0))
                    .sum();
    
    d
}

#[inline(always)]
fn dev_mean(vec: &[u32], mean: &f64) -> Vec<f64> {
    vec.iter()
        .map(|&val| val as f64 - *mean)
        .collect()
}

fn variance(vec: &[u32], mean: &f64) -> f64 {
    let d_mean = dev_mean(vec, mean);
    let n = vec.len() as f64 - 1.0 ;
    let var = sum_of_square(&d_mean) / n;

    var
}

pub fn stdev(vec: &[u32], mean: &f64) -> f64 {
    let var = variance(vec, mean);
    var.sqrt()
}


#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn median_test() {
        let odd: Vec<u32> = vec![1, 4, 3, 5, 6];
        let even: Vec<u32> = vec![1, 4, 3, 5, 6, 6, 8, 10];
        assert_eq!(4.0, median(&odd));
        assert_eq!(5.5, median(&even));
    }
    
    #[test]
    fn var_test() {
        let data: Vec<u32> = vec![1, 4, 3, 5, 6, 6, 8, 10];
        let mean = 5.375;
        
        let exp = 7.982143;
        let res = variance(&data, &mean);
        assert_approx_eq!(exp, res, 6f64);
    }

    #[test]
    fn stdev_test() {
        let data: Vec<u32> = vec![1, 4, 3, 5, 6, 6, 8, 10];
        let mean = 5.375;

        let exp = 2.825269;
        let res = stdev(&data, &mean);
        assert_approx_eq!(exp, res, 6f64);
    }
}