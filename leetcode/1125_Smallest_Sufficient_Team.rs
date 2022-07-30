use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let (skills_n, people_n) = (req_skills.len(), people.len());
        let skills_dic: HashMap<String, usize> = req_skills.into_iter().enumerate().map(|(i, skill)| (skill, i)).collect();
        let mut people_skills_bw: Vec<u16> = people.into_iter()
            .map(|skills| skills.into_iter().fold(0, |acum, skill|  1 << skills_dic[&skill] | acum))
            .collect();
        for i in 0..people_n {
            for j in i + 1..people_n {
                if people_skills_bw[i] | people_skills_bw[j] == people_skills_bw[i] {
                    people_skills_bw[j] = 0;
                }
            }
        }
        let mut dp: HashMap<u16, u64> = HashMap::from([(0, 0)]);
        for person in 0..people_n {
            if people_skills_bw[person] > 0 {
                let dp_slice: Vec<(u16, u64)> = dp.iter().map(|(skills, people)| (*skills, *people)).collect();
                for (skills, people) in dp_slice {
                    let next_skills = skills | people_skills_bw[person];
                    let next_people = (1 << person) | people;
                    if ! dp.contains_key(&next_skills) || dp[&next_skills].count_ones() > next_people.count_ones() {
                        dp.insert(next_skills, next_people);
                    }
                }
            }
        }
        let people = dp[&(((1u32 << skills_n) - 1) as u16)];
        (0..people_n).filter(|person| 1 << *person & people != 0).map(|person| person as i32).collect()
    }
}

#[test]
fn test_1() {
    assert_eq!(
        vec![0,2], 
        Solution::smallest_sufficient_team(
            vec!["java".to_string(),"nodejs".to_string(),"reactjs".to_string()], 
            vec![vec!["java".to_string()],vec!["nodejs".to_string()],vec!["nodejs".to_string(),"reactjs".to_string()]]))
}

#[test]
fn test_2() {
    assert_eq!(
        vec![1,2], 
        Solution::smallest_sufficient_team(
            vec!["algorithms".to_string(),"math".to_string(),"java".to_string(),"reactjs".to_string(),"csharp".to_string(),"aws".to_string()], 
            vec![
                vec!["algorithms".to_string(),"math".to_string(),"java".to_string()],
                vec!["algorithms".to_string(),"math".to_string(),"reactjs".to_string()],
                vec!["java".to_string(),"csharp".to_string(),"aws".to_string()],
                vec!["reactjs".to_string(),"csharp".to_string()],
                vec!["csharp".to_string(),"math".to_string()],
                vec!["aws".to_string(),"java".to_string()]]))
}
