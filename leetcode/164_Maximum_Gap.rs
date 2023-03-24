struct Solution { }

fn lsd_sort(nums: &mut Vec<i32>) {
    let mut vector_1 = nums;
    let mut vector_2 = &mut vec![0; vector_1.len()];

    for i in 0..4 {
        let mut buckets = [0; 256];
        for &num in vector_1.iter() {
            buckets[((num >> (i * 8)) & 255) as usize] += 1;
        }
        let mut pos = 0;
        for i in 0..buckets.len() {
            let tmp = buckets[i];
            buckets[i] = pos;
            pos += tmp;
        }
        for &num in vector_1.iter() {
            let bucket = ((num >> (i * 8)) & 255) as usize;
            vector_2[buckets[bucket]] = num;
            buckets[bucket] += 1;
        }
        let tmp = vector_1;
        vector_1 = vector_2;
        vector_2 = tmp;
    }
}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut max_difference = 0;
        lsd_sort(&mut nums);        
        for i in 1..nums.len() {
            max_difference = max_difference.max(nums[i] - nums[i - 1]);
        }
        max_difference
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::maximum_gap(vec![3,6,9,1]));
    assert_eq!(0, Solution::maximum_gap(vec![10]));
}
