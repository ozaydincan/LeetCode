from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        if len(height) < 3:
            return 0

        left, right = 0, len(height) - 1
        left_max, right_max = height[left], height[right]
        trapped_water = 0

        while left < right:
            if left_max < right_max:
                left += 1
                left_max = left_max if left_max >  height[left] else height[left]
                trapped_water += left_max - height[left]
            else:
                right -= 1
                right_max = right_max  if right_max > height[right] else height[right]
                trapped_water += right_max - height[right]

        return trapped_water


def main():
    height = [0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]
    solution = Solution()
    print(solution.trap(height))


if __name__ == "__main__":
    main()
