impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut seen: HashMap<String, Vec<String>> = HashMap::new();
        for word in strs{
            let mut freq = [0;26];
            for i in word.bytes() {
                freq[(i - b'a') as usize] += 1;
            }

            let key: String = freq.map(|x| x.to_string()).join(",");
            seen.entry(key).or_default().push(word);
        }
        seen.into_values().collect()
    }
}
