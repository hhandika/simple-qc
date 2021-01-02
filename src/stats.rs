//! Heru Handika
//! 1 January 2020
//! Module for statistics

#[inline]
fn sort_vector(vec: &[u32]) -> Vec<u32> {
    let mut sorted_val = vec.to_vec();
    sorted_val.sort_unstable();

    sorted_val
}

pub fn median(val: &[u32]) -> f64 {
    let sorted_val = sort_vector(&val);
    let n = sorted_val.len();
    let hi_midpoint = n / 2;

    if n % 2 == 0  {
        (sorted_val [hi_midpoint - 1]  + sorted_val[hi_midpoint])  as f64 / 2.0
    } else {
        sorted_val[hi_midpoint] as f64
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn median_test() {
        let val: Vec<u32> = vec![1, 4, 3, 5, 6];
        let val_two: Vec<u32> = vec![1, 4, 3, 5, 6, 6, 8, 10];
        assert_eq!(4.0, median(&val));
        assert_eq!(5.5, median(&val_two));
    } 
}