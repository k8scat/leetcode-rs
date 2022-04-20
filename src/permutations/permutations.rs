struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut used = vec![false; nums.len()];
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut p: Vec<i32> = Vec::with_capacity(nums.len());
        Solution::backtrack(&nums, 0, &mut p, &mut res, &mut used);
        return res
    }

    pub fn backtrack(nums: &Vec<i32>, index: usize, p: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
        if index == nums.len() {
            res.push(p.clone());
            return
        }
        for (i, num) in nums.iter().enumerate() {
            if !used[i] {
                used[i] = true;
                p.push(num.clone());
                Solution::backtrack(&nums, index+1, p, res, used);
                p.pop();
                used[i] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::permutations::permutations::Solution;

    #[test]
    fn test_permutations() {
        // let nums: [i32;3] = [1, 2, 3];
        // let nums = nums.to_vec();
        let nums = vec![1, 2, 3];
        let res = Solution::permute(nums);
        println!("{:?}", res)
    }
}
