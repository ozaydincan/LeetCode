class Solution:
    def smallestNumber(self, pattern: str) -> str:
        stack = []
        self.num = []
        for i in range(0, len(pattern) + 1):
            stack.append(i + 1)
            if i == len(pattern) or pattern[i] == "I":
                while stack:
                    self.num.append(str(stack.pop()))
        return "".join(self.num)

