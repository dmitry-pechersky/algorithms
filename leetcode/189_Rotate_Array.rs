struct  Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        fn gcd(mut a: u32, mut b: u32) -> u32 {
            while a % b != 0 {
                let tmp = a % b;
                a = b;
                b = tmp;
            }
            b
        }
        let n = nums.len();
        let k = k as usize % n;
        if k != 0 {
            println!("{}", gcd(n as u32, k as u32));
            for i in 0..gcd(n as u32, k as u32) as usize {
                let mut j = i;
                let mut cur = nums[i];
                loop {
                    let next_j  = (j + k) % n;
                    let temp = nums[next_j];
                    nums[next_j] = cur;
                    cur = temp;
                    j = next_j;
                    if j == i {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod word_dictionary_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![-1,-100,3,99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3,99,-1,-100]);
    }
}