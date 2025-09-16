/// do sort dsa

pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn bb (arr:&mut [i32]){
    let mut swap = true;
    let mut n = arr.len(); // 6 [10,11,12,13,14,1]
                           //   [0, 1, 2, 3, 4, 5]
    while swap{
       swap = false;
       for i in 1..n{ // 6+1 = 7  //1,2,3,4,5
           if arr[i-1] > arr[i]{
                //let temp = arr[i-1];
                (arr[i-1],arr[i]) = (arr[i],arr[i-1]);
            //   arr[i-1] = arr[i];
            //   arr[i] = temp;
               swap = true;
           }
       }
        
    }
   n -=1;
}


#[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_bubble_sort() {
            let mut arr = [64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 34, 64, 90]);
    }
}   
