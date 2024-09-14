from typing import Optional, List


class ListNode:
    def __init__(self, val = 0, next = None):
        self.val = val
        self.next = next


class Solution:

    def spiralMatrix(self, m: int, n: int, head: Optional[ListNode]) -> List[List[int]]:
        matrix = List[List[int]] 


        return matrix

def createLinkedList(array):
    head = ListNode(array[0])
    current = head
    for val in array[1:]:
        current.next = ListNode(val)
        current = current.next
    return head





def main():
    head_values = [3,0,2,6,8,1,7,9,4,2,5,5,0]


