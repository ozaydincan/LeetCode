from typing import List


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        i, j = 0, len(numbers) - 1
        while i < j:
            if numbers[i] + numbers[j] == target:
                return [i + 1, j + 1]
            elif numbers[i] + numbers[j] < target:
                i += 1
            else:
                j -= 1
        return [0, 1]


def main():
    number = [2,3,4]
    target = 6
    solution = Solution()
    idx = solution.twoSum(number, target)
    print("The indices are:")
    for i in idx:
        print(i)


if __name__ == "__main__":
    main()
