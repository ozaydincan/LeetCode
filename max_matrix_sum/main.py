from typing import List


class Solution:
    def maxMatrixSum(self, matrix: List[List[int]]) -> int:
        total_sum = 0
        min_abs_val = float("inf")
        neg_count = 0

        for row in matrix:
            for value in row:
                total_sum += abs(value)
                min_abs_val = min(min_abs_val, abs(value))
                if value < 0:
                    neg_count += 1

        if neg_count % 2 != 0:
            total_sum -= 2 * min_abs_val

        return int(total_sum)


def main():
    matrix = [[1, 2, 3], [-1, -2, -3], [1, 2, 3]]
    sol = Solution()
    output: int = sol.maxMatrixSum(matrix)
    print("The total sum is: ", output)


if __name__ == "__main__":
    main()
