fn count_smaller(l: usize, r: usize, nums: &mut [(u32, i32)], cnts: &mut [i32], buffer: &mut [(u32, i32)]) {
    if l == r {
        return;
    }
    let m = l + (r - l) / 2;
    count_smaller(l, m, nums, cnts, buffer);
    count_smaller(m + 1, r, nums, cnts, buffer);
    let (mut i, mut j, mut k) = (0, l, m + 1);
    while j <= m && k <= r {
        if nums[j].1 > nums[k].1 {
            cnts[nums[j].0 as usize] += (r - k + 1) as i32;
            buffer[i] = nums[j];
            j += 1;
        } else {
            buffer[i] = nums[k];
            k += 1;
        }
        i += 1;
    }
    while j <= m {
        buffer[i] = nums[j];
        j += 1;
        i += 1;
    }
    while k <= r {
        buffer[i] = nums[k];
        k += 1;
        i += 1;
    }
    nums[l..=r].copy_from_slice(&buffer[0..=r-l]);
}

struct Solution {}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums.into_iter()
            .enumerate()
            .map(|(i, num)| (i as u32, num))
            .collect::<Vec<_>>();
        let mut buffer = vec![(0,0); n];
        let mut cnts = vec![0; n];
        count_smaller(0, n - 1, &mut nums, &mut cnts, &mut buffer);
        cnts
    }
}

#[test]
fn test_1() {    
    assert_eq!(vec![2,1,1,0], Solution::count_smaller(vec![5,2,6,1]));
    assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    assert_eq!(vec![0,0,0,0], Solution::count_smaller(vec![5,6,7,8]));
    assert_eq!(vec![2,1,0], Solution::count_smaller(vec![1,0,-1]));
    assert_eq!(vec![2,4,4,2,2,0,0], Solution::count_smaller(vec![0,1,1,0,0,-1,-1]));
}