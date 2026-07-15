impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let set = HashSet::<&i32>::from_iter(&nums);
        set.len() != nums.len()
    }
}
