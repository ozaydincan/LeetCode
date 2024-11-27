from typing import Dict, List


class Solution:
    def findChampion(self, n: int, edges: List[List[int]]) -> int:
        winners: Dict[int, bool] = {team: True for team in range(0, n)}
        for edge in edges:
            winners[edge[1]] = False
        candidates = [team for team, res in winners.items() if res]
        if len(candidates) == 1:
            return candidates[0]
        else:
            return -1
