struct Solution {}

#[derive(Clone)]
struct Lines{
    horizontal: u16,
    vertical: u16,
    diagonal: u16,
    anti_diagonal: u16,
}

impl Lines {
    fn new(horizontal: u16, vertical: u16, diagonal: u16, anti_diagonal: u16) -> Self {
        Self {horizontal, vertical, diagonal, anti_diagonal}
    }

    fn max(&self) -> u16 {
        self.horizontal.max(self.vertical).max(self.diagonal).max(self.anti_diagonal)
    }
}

impl Solution {
    pub fn longest_line(mat: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (mat.len(), mat[0].len());
        let mut dp_1 = vec![Lines::new(0,0,0,0); m];
        let mut dp_2 = vec![Lines::new(0,0,0,0); m];
        let mut max_length = 0;
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    dp_2[j].vertical = dp_1[j].vertical + 1;
                    dp_2[j].horizontal = if j > 0 { dp_2[j - 1].horizontal + 1 } else { 1 };
                    dp_2[j].diagonal = if j > 0 { dp_1[j - 1].diagonal + 1 } else { 1 };
                    dp_2[j].anti_diagonal = if j < m - 1 { dp_1[j + 1].anti_diagonal + 1 } else { 1 };
                    max_length = max_length.max(dp_2[j].max());
                } else {
                    dp_2[j] = Lines::new(0,0,0,0);
                }
            }
            let tmp = dp_1;
            dp_1 = dp_2;
            dp_2 = tmp;
        }
        max_length as i32
    }
}

#[test]
fn test_1() {
    assert_eq!(3, Solution::longest_line(vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,0,0,1]]));
}

#[test]
fn test_2() {
    assert_eq!(4, Solution::longest_line(vec![vec![1,1,1,1],vec![0,1,1,0],vec![0,0,0,1]]));
}

#[test]
fn test_3() {
    assert_eq!(
        9, 
        Solution::longest_line(
            [
                [1,1,0,0,1,0,0,1,1,0],
                [1,0,0,1,0,1,1,1,1,1],
                [1,1,1,0,0,1,1,1,1,0],
                [0,1,1,1,0,1,1,1,1,1],
                [0,0,1,1,1,1,1,1,1,0],
                [1,1,1,1,1,1,0,1,1,1],
                [0,1,1,1,1,1,1,0,0,1],
                [1,1,1,1,1,0,0,1,1,1],
                [0,1,0,1,1,0,1,1,1,1],
                [1,1,1,0,1,0,1,1,1,1]
            ].iter().map(|v| v.to_vec()).collect::<Vec<_>>()        
        )
    );
}