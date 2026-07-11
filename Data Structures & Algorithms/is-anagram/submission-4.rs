impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // hashmap manipulation to an array
        // since the constraint of lowercase english
        // letters is present, instead of allocating 
        if s.len() != t.len(){
            return false;
        }
        let mut hm = [0;26];
        let c1 = s.as_bytes();
        let c2 = t.as_bytes();

        for c in c1{
            hm[(c - b'a') as usize] += 1
        }

        for c in c2 {
            hm[(c - b'a') as usize] -= 1
        }

        for v in hm {
            if v != 0 {
                return false
            }
        }
        true
    }
}
