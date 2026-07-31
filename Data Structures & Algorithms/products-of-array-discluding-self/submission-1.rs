impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // brute force
        let mut res = Vec::new();
        for i in 0..nums.len(){
            let mut ans = 1;
            for j in 0..nums.len(){
                if j != i{
                    ans*= nums[j]
                }
            }
            res.push(ans)
        }
        res
    }
}
