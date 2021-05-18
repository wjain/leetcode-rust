pub struct Solution;

impl Solution {
    #[cfg(feature = "fori")]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for i in 0..nums.len() - 1 {
            for j in i+1..nums.len() {
                if target == nums[i] + nums[j] {
                    res = vec![i as i32, j as i32];
                    break;
                }
            }
        }

        res
    }

    #[cfg(feature = "map")]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut res: Vec<i32> = vec![];
        let mut h: HashMap<i32, i32> = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let key = target - n;
            match h.get(&key) {
                Some(v) => {
                    res.push(*v);
                    res.push(i as i32);
                    break;
                },
                None => {
                    h.insert(*n, i as i32);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4, 8], 6));
        assert_eq!(vec![0, 2], Solution::two_sum(vec![3, 2, 3], 6));
    }
}
