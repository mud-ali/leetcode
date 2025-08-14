#include <string.h>
#include <stdlib.h>

char* largestGoodInteger(char* num) {
    int largest = -1;
    int i=0;
    while (i <= strlen(num)-3) {
        if (num[i]==num[i+1] && num[i+1]==num[i+2]) {
            int val = num[i]-'0';
            if (val > largest) largest = val;
        }
        i++;
    }

    char* out = malloc(sizeof(char) * 4);
    if (largest == -1) 
        sprintf(out, "");
    else 
        sprintf(out, "%d%d%d", largest, largest, largest);
    return out;
}