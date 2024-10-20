from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        result = []
        stack = [(0, 0, "")]

        while stack:
            open_count, close_count, current_string = stack.pop()
            if len(current_string) == 2 * n:
                result.append(current_string)
                continue

            if open_count < n:
                stack.append((open_count + 1, close_count, current_string + "("))

            if close_count < open_count:
                stack.append((open_count, close_count + 1, current_string + ")"))

        return result


def main():
    n = 3
    solution = Solution()
    print(solution.generateParenthesis(n))


if __name__ == "__main__":
    main()
