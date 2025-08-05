class Solution {
    fun numOfUnplacedFruits(fruits: IntArray, baskets: IntArray): Int {
        var unplaced = 0
        for (fruit in fruits) {
            for (basketIndex in baskets.indices) {
                if (baskets[basketIndex] >= fruit) {
                    baskets[basketIndex] = -1
                    unplaced--
                    break
                }
            }
            unplaced++
        }
        return unplaced
    }
}