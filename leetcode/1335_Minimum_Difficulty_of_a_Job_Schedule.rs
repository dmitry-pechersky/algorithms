pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);
        
        if n < d {
            return -1;
        }

        let (mut dp_1, mut dp_2) = (vec![0; n], vec![0; n]);
        dp_1[n - 1] = job_difficulty[n - 1];
        for i in (0..n - 1).rev() {
            dp_1[i] = dp_1[i + 1].max(job_difficulty[i]);
        }

        for days in 2..d + 1 {
            for i in 0..n - days + 1 {
                let mut max_dif = i32::MIN;
                let mut min = i32::MAX;
                for j in i..n - days + 1 {
                    max_dif = max_dif.max(job_difficulty[j]);
                    min = min.min(max_dif + dp_1[j + 1]);
                }
                dp_2[i] = min;
            }
            std::mem::swap(&mut dp_1, &mut dp_2);
        }
        
        dp_1[0]
    }
}

#[test]
fn test_min_difficulty() {
    assert_eq!(7, Solution::min_difficulty(vec![2,6,7], 1));
    assert_eq!(9, Solution::min_difficulty(vec![2,6,7], 2));
    
    assert_eq!(7, Solution::min_difficulty(vec![6,5,4,3,2,1], 2));
    assert_eq!(-1, Solution::min_difficulty(vec![9,9,9], 4));
    assert_eq!(3, Solution::min_difficulty(vec![1,1,1], 3));
    
    assert_eq!(
        16, 
        Solution::min_difficulty(
            vec![1,4,3,1,1,9,1,5,1,9], 
            5)
    );
    assert_eq!(
        16, 
        Solution::min_difficulty(
            vec![1,4,3,1,1,9,7,1,1,7,4,7,2,8,9,4,3,2,2,1,8,4,6,4,7,7,7,5,9,1,4,4,5,9], 
            5)
    );
    assert_eq!(
        14, 
        Solution::min_difficulty(
            vec![1,4,3,1,1,9,7,1,7,9], 
            3)
    );
}