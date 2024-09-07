#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> numMap; // To store the value and its index
        for (int i = 0; i < nums.size(); ++i) {
            int complement = target - nums[i];
            if (numMap.find(complement) != numMap.end()) {
                return {numMap[complement], i}; // Return indices of the two numbers
            }
            numMap[nums[i]] = i;
        }
        return {}; // If no solution is found, though the problem guarantees one solution
    }
};


int main() {
    vector<int> nums = {3,2,1,5,8,7};
    cout << *nums.begin();
    Solution solution;
    vector<int> ids = solution.twoSum(nums, 9);
    for(int i:ids){
        cout << i<<endl;
    }
    return 0;
}
