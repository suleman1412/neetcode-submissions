class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        """
        hashmap
        seen  = create a dict that stores the num as the key and the idx as the value

        iterate over the nums
        calc the difference between target and the current i
        check if the difference lies in seen
        """

        seen = {}
        for idx, n in enumerate(nums):
            diff = target - n
            if diff in seen:
                return [seen[diff], idx]
            else:
                seen[n] = idx