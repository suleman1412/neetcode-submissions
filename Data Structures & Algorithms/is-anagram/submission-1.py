class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        # if len dont match then false
        # make a freq table of each string
        # and compare the tables

        # sort all in the same order
        # both result of sort should match

        if len(s) != len(t):
            return False

        freq = {}
        for i in s:
            if i not in freq:
                freq[i] = 1
            else:
                freq[i] += 1
        
        for j in t:
            if j in freq:
                freq[j] -= 1
            else:
                return False
        
        for i in freq:
            if freq[i] != 0:
                return False
        
        return True
