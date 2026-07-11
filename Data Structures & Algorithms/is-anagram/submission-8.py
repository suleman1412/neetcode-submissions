class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        """
        implementation of direct mapped frequency array in py
        this is still worse than rs because python
        stores everythiong on the heap regardless
        but still better than using Dicts because no 
        hashing overhead
        """
        if len(s) != len(t):
            return False
        
        hm = [0] * 26
        for i in s:
            hm[ord(i) - ord('a')] += 1
        
        for j in t:
            hm[ord(j) - ord('a')] -= 1
        
        for v in hm:
            if v != 0:
                return False
        
        return True