#include <vector>

using namespace std;

class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& nums) {
        vector<bool> ans(nums.size());
        unsigned long long current = 0;
        int i = 0;
        for (int n: nums) {
            current = (current << 1) | n;
            if (current % 5 == 0) {
                ans.at(i)=true;
                current = 0;
            }
            else ans.at(i) = false;
            i++;
        }
        return ans;
    }
};