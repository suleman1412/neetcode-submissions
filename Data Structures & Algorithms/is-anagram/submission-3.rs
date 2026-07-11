impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // hashset rs
        if s.len() != t.len(){
            return false;
        }
        let mut freq = HashMap::new();
        
        for i in s.chars() {
            if !freq.contains_key(&i){
                freq.insert(i,1);
            } else {
                if let Some(count) = freq.get_mut(&i){
                    *count += 1;
                }
            }
        }

        for j in t.chars(){
            if freq.contains_key(&j){
                if let Some(count) = freq.get_mut(&j){
                    *count -= 1;
                }
            } else {
                return false;
            }
        }

        for (_, count) in &freq{
            if *count != 0 {
                return false;
            }
        }
        return true;
    }
}
