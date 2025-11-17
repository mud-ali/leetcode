#include <string>
#include <vector>

class Solution {
public:
    string convert(string s, int numRows) {
        vector<string> total(numRows+1);
        if (numRows == 1) return s;
        for (int i=0;i<s.length();i++) {
            int halfway = (numRows-1);
            int spot = i % (2*(numRows-1));
            int dist = abs(spot-halfway);

            total.at(numRows-dist).push_back(s.at(i));
        }

        string zig = "";
        for (string n:total) {
            zig += n;
        }
        return zig;
    }
};