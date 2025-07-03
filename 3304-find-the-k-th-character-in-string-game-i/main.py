class Solution:
    def kthCharacter(self, k: int) -> str:
        word = 'a'

        def nextWord(init: str) -> str:
            out = ''
            for char in init:
                if char == 'z': out+= 'a'
                else: out += chr(ord(char)+1)
            return out

        while len(word) < k:
            word += nextWord(word)
        return word[k-1]