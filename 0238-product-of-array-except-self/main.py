class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prefixes = [nums[0]]
        for i in range(1,len(nums)-1):
            prefixes.append(prefixes[i-1]*nums[i])
        
        suff = [nums[len(nums)-1]]
        for i in range(len(nums)-2, 0, -1):
            suff.append(suff[len(nums)-2-i]*nums[i])

        suff = suff[::-1]

        ans = []
        for i in range(len(nums)):
            f1 = 1 if i==0 else prefixes[i-1]
            f2 = 1 if i==len(nums)-1 else suff[i]
            ans.append(f1 * f2)
        return ans