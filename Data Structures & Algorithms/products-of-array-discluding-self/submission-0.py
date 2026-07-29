class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        # brute force
        res = []
        for i in range(0, len(nums)):
            ans = 1
            for j in range(0, len(nums)):
                if j != i:
                    ans *= nums[j]
            
            res.append(ans)
        
        print(res)
        return res
