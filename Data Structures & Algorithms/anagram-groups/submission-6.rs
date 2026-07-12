impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        /*
            easy approach
            sort all words and append words that have the same sort
        */
        let mut seen: HashMap<String, Vec<String>> = HashMap::new();
        for word in strs{
            let mut sorted_chars: Vec<char> = word.chars().collect();
            sorted_chars.sort();
            let sorted_word: String = sorted_chars.into_iter().collect();
            seen.entry(sorted_word).or_default().push(word);
        }
        seen.into_values().collect()
    }
}
