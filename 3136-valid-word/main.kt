class Solution {
    fun isValid(word: String): Boolean {
        if (word.length < 3) return false
        if (!word.toCharArray().any{ c -> "aeiouAEIOU".contains(c) }) return false

        if (word.toCharArray().all{ c -> "aeiouAEIOU1234567890".contains(c) }) return false
    
        return (word.toCharArray().all{ c -> c.isLetterOrDigit() })
    }
}