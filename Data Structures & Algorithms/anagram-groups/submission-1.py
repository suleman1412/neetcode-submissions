class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        """
            anagrams are freq tables
            freq tables for strings of lowercase english letter is just an array of 26 elements
            count of the letter at position idx for the letter that comes up in the word
                return a tuple as they are immutable

            make a hashmap of seen freq tables, key as the freq and value as array of strings
            when first seen create the array with just the string
            if another string has the same freq table, append the word to the value
        """
        def getFreqTable(word: str):
            freq = [0] * 26
            for i in word:
                freq[ord(i) - ord('a')] += 1
            return tuple(freq)
        
        seen = {}
        for word in strs:
            freq = getFreqTable(word)
            if freq not in seen:
                seen[freq] = [word]
            else:
                seen[freq].append(word)
        return list(seen.values())
        