#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    int longestNiceSubarray(vector<int>& nums) {
        int max_len = 0;
        int left = 0;
        int bitmask = 0;
        
        for (int right = 0; right < nums.size(); right++) {
            while ((bitmask & nums[right]) != 0) {
                bitmask ^= nums[left];
                left++;
            }
            bitmask |= nums[right];
            max_len = max(max_len, right - left + 1);
        }
        
        return max_len;
    }
};

int main() {
    vector<int> nums = {1, 3, 8, 48, 10};
    Solution sol;
    cout << "Result: " << sol.longestNiceSubarray(nums) << '\n'; // Output: 3
    return 0;
} 
