impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        hashmap
        seen  = create a dict that stores the num as the key and the idx as the value

        iterate over the nums
        calc the difference between target and the current i
        check if the difference lies in seen
        */
        let mut seen = HashMap::new();
        for (idx, num) in nums.iter().enumerate(){
            let index = idx as i32;
            let diff = target - num;
            if let Some(i) = seen.get(&diff){
                return vec![*i, index];
            }
            else {
                seen.insert(num, index);
            }
        }
        return vec![];
    }
}
