impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut seen: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for word in strs{
            let mut freq = [0;26];
            for i in word.bytes() {
                freq[(i - b'a') as usize] += 1;
            }
            seen.entry(freq).or_insert(Vec::new()).push(word);
        }
        seen.into_values().collect()
    }
}
