class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq_table = {}
        max
        for i in nums:
            if i not in freq_table:
                freq_table[i] = 1
            else:
                freq_table[i] += 1
        
        buckets = [[] for _ in range(len(nums) + 1)]
        print(buckets)

        for key, value in freq_table.items():
            buckets[value].append(key)
        
        res = []
        for i in range(len(buckets) - 1, 0, -1):
            for num in buckets[i]:
                res.append(num)
                if len(res) == k:
                    return res