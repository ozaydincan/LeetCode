from typing import List

class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        n:int = len(nums)
        if not nums:
            return[-1, -1]
        def binarySearch(flag: bool):
            left, right = 0, n -1
            result = -1
            while left <= right:
                mid:int = (left + right)// 2
                if nums[mid] > target:
                    right = mid - 1
                elif nums[mid] < target:
                    left = mid + 1
                else:
                    result = mid
                    if flag:
                        right = mid - 1
                    else:
                        left = mid + 1
            return result

        return [binarySearch(True), binarySearch(False)]

