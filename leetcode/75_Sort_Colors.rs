struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnts = [0usize; 3];
        for i in 0..nums.len() {
            cnts[nums[i] as usize] += 1;
        }
        let mut j = 0;
        for i in 0..cnts.len() {
            while cnts[i] > 0 {
                nums[j] = i as i32;
                cnts[i] -= 1;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod sort_colors_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
    }    

    #[test]
    fn test_2() {
        let mut nums = vec![2,0,1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }    

    #[test]
    fn test_3() {
        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
    }    

    #[test]
    fn test_4() {
        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }    

}