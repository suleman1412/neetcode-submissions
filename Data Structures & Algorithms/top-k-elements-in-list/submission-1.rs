impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_table = HashMap::new();
        for &n in &nums {
            *freq_table.entry(n).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        
        for (num, freq) in freq_table {
            buckets[freq].push(num);
        }

        // Step 3: Iterate backward to collect top k elements
        let mut res = Vec::with_capacity(k as usize);
        
        // .into_iter().rev() iterates from the end of the vector to the beginning
        for bucket in buckets.into_iter().rev() {
            for num in bucket {
                res.push(num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        
        res
    }
}