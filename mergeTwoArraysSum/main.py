from typing import Dict, List


class Solution:
    def mergeArrays(
        self, nums1: List[List[int]], nums2: List[List[int]]
    ) -> List[List[int]]:
        id_map:Dict[int, int] = {}

        for key, val in nums1:
            id_map[key] = id_map.get(key, 0) + val

        for key, val in nums2:
            id_map[key] = id_map.get(key, 0) + val

        return [list(item) for item in sorted(id_map.items())] 
