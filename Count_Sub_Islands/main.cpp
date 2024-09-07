#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    bool dfs(int i, int j, vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        if (i < 0 || i >= grid1.size() || j < 0 || j >= grid1[0].size() || grid2[i][j] == 0) {
            return true;
        }
        if (grid1[i][j] == 0 && grid2[i][j] == 1) {
            return false;
        }
        grid2[i][j] = 0;
        bool isSubIsland = true;
        int directions[4][2] = {{1,0}, {0,1}, {-1,0}, {0,-1}};
        for (int d = 0; d < 4; ++d) {
            int ni = i + directions[d][0];
            int nj = j + directions[d][1];
            if (!dfs(ni, nj, grid1, grid2)) {
                isSubIsland = false;
            }
        }
        return isSubIsland;
    }

    int countSubIslands(vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        int m = grid1.size();
        int n = grid1[0].size();
        int subIslandCount = 0;

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid2[i][j] == 1) {
                    if (dfs(i, j, grid1, grid2)) {
                        subIslandCount++;
                    }
                }
            }
        }
        
        return subIslandCount;
    }
};

int main() {
    vector<vector<int>> grid1 = {
        {1, 1, 1, 0, 0},
        {0, 1, 1, 1, 1},
        {0, 0, 0, 0, 0},
        {1, 0, 0, 0, 0},
        {1, 1, 0, 1, 1}
    };

    vector<vector<int>> grid2 = {
        {1, 1, 1, 0, 0},
        {0, 0, 1, 1, 1},
        {0, 1, 0, 0, 0},
        {1, 0, 1, 1, 0},
        {0, 1, 0, 1, 0}
    };

    Solution solution;
    int result = solution.countSubIslands(grid1, grid2);

    cout << "Number of sub-islands: " << result << endl;

    return 0;
}

