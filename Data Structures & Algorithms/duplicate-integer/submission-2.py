class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        set_nums = set(nums)
        if len(set_nums) == len(nums):
            return False
        for i in nums:
            if i not in set_nums:
                set_nums.add(i)
            else:
                return True

            return False