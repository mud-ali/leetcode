class Solution:
    def kLengthApart(self, nums: List[int], k: int) -> bool:
        last_1 = None
        for i in range(len(nums)):
            if nums[i] == 1:
                if last_1 is not None:
                    dist = i-last_1
                    if dist < k: return False
                last_1 = i+1
        return True
        