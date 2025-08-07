class Solution:
    def findLucky(self, arr: List[int]) -> int:
        luck = -1
        for a in arr:
            if a > luck and arr.count(a) == a:
                luck = a
        return luck