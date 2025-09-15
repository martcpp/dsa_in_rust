fn main() {
    println!("Hello, world!");
    let pinf = f64::INFINITY;
    let ninf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    let ed  = f64::EPSILON;
    println!("pinf: {}, ninf: {}, nan: {}, ed: {}", pinf, ninf, nan, ed);
}


fn find_min(nums: &[f64]) -> Option<f64> {
    let mut min = f64::INFINITY;
    match nums.len() {
        0 => None,
        _ => {
            for &num in nums {
                if num.is_nan() {
                    return None;
                }
                if num < min {
                    min = num;
                }
            }
            Some(min)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min() {
        let nums = [3.5, 2.1, 4.7,f64::INFINITY, -1.0, f64::NAN];
        assert_eq!(find_min(&nums), None);
        let nums2 = [3.5, 2.1, 4.7, -1.0];
        assert_eq!(find_min(&nums2), Some(-1.0));
        let nums3: [f64; 0] = [];
        assert_eq!(find_min(&nums3), None);
    }
}