from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        length = len(nums)
        left, right = 0, length -1
        while left <= right:
            mid = left + (right - left) // 2
            if nums[mid] == target:
                return mid
            elif nums[mid] >= nums[left]:
                if target >= nums[left] and target < nums[mid]:
                    right = mid - 1
                else:
                    left = mid + 1
            else:
                if target <= nums[right] and target > nums[mid]:
                    left = mid + 1
                else:
                    right = mid + 1

        return -1


def main():
    nums:List[int] = [4,5,6,7,0,1,2]
    target:int = 0 
    sol = Solution()
    output:int = sol.search(nums, target)
    print(f"The target integer {target} is in index {output}")


if __name__ == "__main__":
    main()
