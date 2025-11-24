#include <vector>
using namespace std;

class Solution {
public:
    int findFinalValue(vector<int>& nums, int original) {
        int i=0;
        int search = original;
        while (i<nums.size()) {
            if (nums.at(i)==search) {
                search *= 2;
                i=0;
                continue;
            }

            i++;
        }
        return search;
    }
};