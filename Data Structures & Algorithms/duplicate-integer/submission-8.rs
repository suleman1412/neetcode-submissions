impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let s1 = HashSet::<&i32>::from_iter(&nums); 
        s1.len() != nums.len()
    }
}
