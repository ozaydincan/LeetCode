from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        result = [0] * len(temperatures)
        stack = list()
        for i, _temp in enumerate(temperatures):
            while stack and temperatures[i] > temperatures[stack[-1]]:
                idx = stack.pop()
                result[idx] = i - idx
            stack.append(i)
        return result


def main():
    temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
    solution = Solution()
    print(solution.dailyTemperatures(temperatures))


if __name__ == "__main__":
    main()
