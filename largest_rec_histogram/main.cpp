#include <algorithm>
#include <atomic>
#include <iostream>
#include <iterator>
#include <vector>

class Solution {
public:
    int largestRectangleArea(std::vector<int> &heights) {
        int n = heights.size();
        std::vector<int> stack = {-1};
        int max_area = 0;

        for (int i = 0; i < n; i++) {
            while (stack.back() != -1 && heights[stack.back()] >= heights[i]) {
                int curr_h = heights[stack.back()];
                stack.pop_back();
                int curr_w = i - stack.back() - 1;
                max_area = std::max(max_area, curr_h * curr_w);
            }
            stack.push_back(i);
        }

        while (stack.back() != -1) {
            int curr_h = heights[stack.back()];
            stack.pop_back();
            int curr_w = n - stack.back() - 1;
            max_area = std::max(max_area, curr_h * curr_w);
        }

        return max_area;
    }
};

std::ostream &operator<<(std::ostream &os, const std::vector<int> &vec) {
  os << "[";
    if (!vec.empty()) {
        std::copy(vec.begin(), vec.end()-1, std::ostream_iterator<int>(os, ", "));
    os << vec.back();
    }
  os << "]";
    return os;
}

int main() {
    std::vector<int> heights = {2, 1, 5, 6, 2, 3};
    Solution sol;
    std::cout << "The rectangle with the maximum area in vector " << heights
              << " is " << sol.largestRectangleArea(heights) << std::endl;
    return 0;
}

