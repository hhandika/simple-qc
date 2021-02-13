//! Heru Handika
//! 1 January 2020
//! Module for statistics

use std::cmp::Reverse;

#[inline]
fn sort_vector_asc(vec: &[u32]) -> Vec<u32> {
    let mut sorted_vec = vec.to_vec();
    sorted_vec.sort_unstable();

    sorted_vec
}

pub fn median(vec: &[u32]) -> f64 {
    let sorted_vec = sort_vector_asc(&vec);
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
    
    sum_of_square(&d_mean) / n
}

pub fn stdev(vec: &[u32], mean: &f64) -> f64 {
    let var = variance(vec, mean);
    var.sqrt()
}

#[allow(dead_code)]
fn sort_vec_desc(vec: &[u32]) -> Vec<u32> {
    let mut sorted_vec = vec.to_vec();
    sorted_vec.sort_by_key(|v| Reverse(*v));

    sorted_vec
}

#[allow(dead_code)]
fn cumsum(vec: &[u32]) -> Vec<u32> {
    let mut csum = Vec::new();
    let mut sum = 0;
    vec.iter()
        .for_each(|v|{
            sum += v;
            csum.push(sum);
        });
    
    csum
}

#[allow(dead_code)]
fn n2_stats(contigs: &[u32]) -> usize {
    let n2 = contigs.iter().sum::<u32>() / 2;
    
    n2 as usize
}

fn get_n50_idx(csum_contigs: &[u32], n2: u32) -> usize {
    csum_contigs.iter()
        .position(|i| *i >= n2)
        .unwrap()
}

#[allow(dead_code)]
pub fn n50_stats(contigs: &[u32]) -> u32 {
    let sorted_contigs = sort_vec_desc(contigs);
    let csum_contigs = cumsum(&sorted_contigs);
    let n2 = n2_stats(contigs);
    let idx = get_n50_idx(&csum_contigs, n2 as u32);
    
    sorted_contigs[idx]
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

    #[test]
    fn csum_test() {
        let a = vec![1, 2, 3];
        let res = vec![1, 3, 6];

        assert_eq!(res, cumsum(&a));
    }

    #[test]
    fn sorted_vec_desc_test() {
        let a = vec![1, 2, 3];
        let res = vec![3, 2, 1];

        assert_eq!(res, sort_vec_desc(&a));
    }

    #[test]
    fn n2_test() {
        let a = vec![2,3,4,5,6,7,8,9,10];
        let b = vec![2,3,4,5,6,7,8,9,10,11];

        assert_eq!(27, n2_stats(&a));
        assert_eq!(32, n2_stats(&b));
    }

    #[test]
    fn n50_stats_test() {
        let contigs = vec![2,3,4,5,6,7,8,9,10];

        assert_eq!(8, n50_stats(&contigs));
    }
}