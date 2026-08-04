class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [1] * len(nums)
        pre, post = 1, 1

        # Calculate multiplication of all left 
        for i in range(len(nums)):
            res[i] = pre
            pre *= nums[i]
        
        # running product while keeping the prefix
        for i in range(len(nums) - 1, -1, -1):
            res[i] *= post
            post *= nums[i]
        
        return res