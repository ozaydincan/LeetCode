from typing import List


class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        if not nums:
            return 0

        longest: int = 0
        num_set = set(nums)
        current: int = 1

        for num in num_set:
            if num - 1 not in num_set:
                current_num = num
                current = 1
                while current_num + 1 in num_set:
                    current_num += 1
                    current += 1
                longest = longest if longest > current else current

        return longest


def main():
    nums: List[int] = [100, 4, 200, 1, 3, 2]
    solution = Solution()
    result = solution.longestConsecutive(nums)
    print(f"The result is: {result}")


if __name__ == "__main__":
    main()
