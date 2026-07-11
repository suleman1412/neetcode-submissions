class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        """
        implementation of direct mapped frequency array in py
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