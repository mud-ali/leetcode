class Solution {
    fun sortColors(nums: IntArray): Unit {
        val amount = IntArray(3) { 0 }
        for (num in nums) {
            amount[num]++
        }
        var next = 0
        for (i in 0..2) {
            while (amount[i] > 0) {
                amount[i]--
                nums[next++] = i
            }
        }
    }
}