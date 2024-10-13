from pprint import pprint
from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        result: List[List[int]] = []
        nums.sort()
        for i in range(len(nums)):
            if i > 0 and nums[i] == nums[i - 1]:
                continue
            target = -nums[i]
            seen, used = set(), set()

            for j in range(i + 1, len(nums)):
                diff = target - nums[j]
                if diff in seen and (nums[j], diff) not in used:
                    result.append([nums[i], diff, nums[j]])
                    used.add((nums[j], diff))
                seen.add(nums[j])

        return result


def main():
    nums = [-1, 0, 1, 2, -1, -4]
    solution = Solution()
    triplets = solution.threeSum(nums)
    print(triplets)
    pprint(triplets)

if __name__ == "__main__":
    main()
