from typing import List


class Solution:
    def maxArea(self, heights: List[int]) -> int:
        max_result = 0
        left, right = 0, len(heights) - 1
        while left < right:
            if heights[left] < heights[right]:
                curr_area = heights[left] * (right - left)
                left += 1
            else:
                curr_area = heights[right] * (right - left)
                right -= 1
            max_result = max_result if max_result > curr_area else curr_area
        return max_result


def main():
    height = [1, 7, 2, 5, 4, 7, 3, 6]
    solution = Solution()
    max_area = solution.maxArea(height)
    print(f"The max area is: {max_area}")


if __name__ == "__main__":
    main()
