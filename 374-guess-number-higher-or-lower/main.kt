/** 
 * The API guess is defined in the parent class.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * fun guess(num:Int):Int {}
 */

class Solution:GuessGame() {
    override fun guessNumber(n:Int):Int {
        var guessMin = 1
        var guessMax = n+1
        var lastGuess = ((guessMax - guessMin) / 2) + guessMin
        var result = guess(lastGuess)

        while (result != 0) {
            if (result == 1) {
                guessMin = lastGuess
            } else {
                guessMax = lastGuess
            }
            lastGuess = ((guessMax - guessMin) / 2) + guessMin
            result = guess(lastGuess)
        }
        return lastGuess
    }
}