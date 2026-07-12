class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        """
        hashmap two pass
        """
        seen = {}
        for idx, n in enumerate(nums):
            seen[n] = idx
        
        for idx, n in enumerate(nums):
            diff = target - n
            if diff in seen and seen[diff] != idx:
                return [idx, seen[diff]]
        
        return []
