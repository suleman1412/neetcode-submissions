impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        /* 
        direct mapped frequeuncy array
        hashmap manipulation to an array
        since the constraint of lowercase english
        letters is present, instead of allocating 
        a Hashmap, we create a hacky version of it using 
        array by defining only 26 element long array, 
        containing only 0s

        convert the given strings to bytes, to get
        an array of ascii numbers of each character

        subtract with 'a' as thats a given a constraint,
        and only for that index increment the values

        run the loop back and this time subtract the values
        if they are present in t

        then check

        its simple how one would do with a hashmap solution
        but better because this by givign a definite size of
        hm, we allocate it on the stack.
        */
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
