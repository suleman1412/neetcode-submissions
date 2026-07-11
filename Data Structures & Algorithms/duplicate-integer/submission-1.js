class Solution {
    /**
     * @param {number[]} nums
     * @return {boolean}
     */
    hasDuplicate(nums) {
        const set_nums = new Set()
        for (let i = 0 ; i <= nums.length; i++){
            if(set_nums.has(nums[i])){
                return true
            }

            set_nums.add(nums[i])
        }
        return false
    }

}
