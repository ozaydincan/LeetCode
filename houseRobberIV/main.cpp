#include <vector>

using namespace std;

class Solution {
public:
  int minCapability(vector<int> &nums, int k) {
    int left = *min_element(nums.begin(), nums.end()),
        right = *max_element(nums.begin(), nums.end());

    while (left < right) {
      int mid = left + (right - left) / 2, count = 0;
      for (int i = 0; i < nums.size(); i++) {
        if (nums[i] <= mid) {
          count++, i++;
        }
      }
      (count >= k) ? right = mid : left = mid + 1;
    }

    return left;
  }
};

int main(int argc, char *argv[]) {
  Solution sol;
  return 0;
}
