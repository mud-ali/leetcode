class Solution {
public:
    int countCollisions(string directions) {
        int n = directions.length();
        int collisions = 0;
        bool toLeft = false;
        for (int i=0;i<n;i++) {
            if (directions.at(i)=='S') {
                toLeft = true;
                continue;
            }
            if (directions.at(i)=='R') {
                toLeft = true;
                for (int j=i+1;j<n;j++) {
                    if (directions.at(j)!='R') {
                        collisions++;
                        break;
                    }
                }
            } else if (directions.at(i)=='L') {
                if (toLeft) collisions++;
            }
        }
        return collisions;
    }
};