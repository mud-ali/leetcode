#include <ctype.h>

class Solution {
public:
    char toUpper(char c) {
        if (c >= 'a' && c <= 'z') {
             return c - 32;
        }
        return c;
    }

    bool isPalindrome(string s) {
        int i = 0;
        int i2 = s.length() - 1;

        while (i2 > i) {
            while (!isalnum(s[i]) && i < i2) i++;
            while (!isalnum(s[i2]) && i < i2) i2--;

            if (toUpper(s[i])!=toUpper(s[i2])) return false;
            i2--;
            i++;
        }
        return true;
    }
};