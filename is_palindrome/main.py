import re

class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = re.sub(r"[^a-zA-Z0-9]", '', s).lower()
        i, j = 0, len(s)-1
        while i < j:
            if s[i] != s[j]:
                return False
            i += 1
            j -= 1
        return True

def main():
    s = "Was it a car or a cat I saw?"
    solution = Solution()
    result = solution.isPalindrome(s)
    print(f"The string: '{s}' is a palindrome: \n{result}")


if __name__ == "__main__":
    main()

