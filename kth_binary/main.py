class Solution:
    def findKthBit(self, n: int, k: int) -> str:
        pos = k& -k
        inverted = ((k // pos)>>1 & 1) == 1
        original = (k&1) == 0
        if inverted:
            return "0" if original else "1"
        else:
            return "1" if original else"0"
print(Solution.findKthBit(self=Solution(), n=4, k=8))
