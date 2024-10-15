class Solution:
    def minimumSteps(self, s: str) -> int:
        one_count = 0
        min_steps = 0

        for char in s:
            if char == "1":
                one_count += 1
            elif char == "0":
                min_steps += one_count
        return min_steps


def main():
    s = "101"
    solution = Solution()
    print(f"The minimum necessary step is: {solution.minimumSteps(s)}")


if __name__ == "__main__":
    main()
