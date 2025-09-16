mod sort;
use sort::bubble_sort;
fn main() {
    // println!("Hello, world!");
    // let pinf = f64::INFINITY;
    // let ninf = f64::NEG_INFINITY;
    // let nan = f64::NAN;
    // let ed  = f64::EPSILON;
    // println!("pinf: {}, ninf: {}, nan: {}, ed: {}", pinf, ninf, nan, ed);

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    print!("Before sorting: {:?}", arr);
    bubble_sort(&mut arr);
    print!("After sorting: {:?}", arr);
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


fn find_longest_str(words:&[&str]) -> Option<String>{
    let mut longest =  String::new();
    
    match words.len(){
        0 => None,
        _ => {
            for word in words.iter(){
                if word.len() > longest.len(){
                    longest = word.to_string();
                }
            }
            Some(longest)
        }
    }
}

fn resvers_a_string(word:&str) -> String{
    let mut revser = String::new();
    
    if word.is_empty(){
        return word.to_string()
    }

    let array_wp:Vec<char> =  word.chars().collect();
    let length =  word.len()-1; //GOD 3-1 2
    for i in 0..=length{ //0 1 2
        revser.push(array_wp[length-i]); 
       // length -1;
    }
    revser
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
    #[test]
    fn test_find_longest_str(){
        let words = ["apple", "banana", "cherry", "date"];
        assert_eq!(find_longest_str(&words), Some("banana".to_string()));
        let words2: [&str;0] = [];
        assert_eq!(find_longest_str(&words2), None);
    }
    #[test]
    fn test_resvers_a_string(){
        let word = "hello";
        assert_eq!(resvers_a_string(word), "olleh".to_string());
        let word2 = "";
        assert_eq!(resvers_a_string(word2), "".to_string());
    }   
}