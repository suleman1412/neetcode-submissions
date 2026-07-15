impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (idx, num) in nums.iter().enumerate(){
            let index = idx as i32;
            let diff = target - num;
            if let Some(i) = seen.get(&diff){
                return vec![*i, index];
            }
            else {
                seen.insert(*num, index);
            }
        }
        return vec![];
    }
}
