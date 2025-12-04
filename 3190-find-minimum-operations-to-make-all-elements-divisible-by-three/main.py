class Solution:
    def minimumOperations(self, nums: List[int]) -> int:
        t = 0
        for n in nums:
            if n % 3 != 0: t+= 1
        return t