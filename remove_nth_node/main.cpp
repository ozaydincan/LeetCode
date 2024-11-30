#include <iostream>
#include <ostream>
#include <vector>

// Definition for singly-linked list.
struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
  ListNode *removeNthFromEnd(ListNode *head, int n) {
    int i = 0;
    ListNode *curr = head;
    while (curr != nullptr) {
      curr = curr->next;
      i++;
    }
    if (n == i) {
      ListNode *newHead = head->next;
      delete head;
      return newHead;
    }

    curr = head;
    for (int j = 0; j < i - n - 1 && curr != nullptr; j++) {
      curr = curr->next;
    }

    if (curr != nullptr && curr->next != nullptr) {
      ListNode *nodeToDelete = curr->next;
      curr->next = curr->next->next;
      delete nodeToDelete;
    }

    return head;
  }

  void printList(ListNode *head) {
    ListNode *dummy = head;
    std::cout << "List is ";
    while (dummy != nullptr) {
      std::cout << dummy->val << ',';
      dummy = dummy->next;
    }
    std::cout << std::endl;
  }
};

ListNode *buildList(const std::vector<int> &values) {
  if (values.empty())
    return nullptr;
  ListNode *head = new ListNode(values[0]);
  ListNode *current = head;
  for (size_t i = 1; i < values.size(); ++i) {
    current->next = new ListNode(values[i]);
    current = current->next;
  }
  return head;
}

int main(int argc, char *argv[]) {
  std::vector<int> list = {1, 2, 3, 4, 5};
  ListNode *linked_list = buildList(list);
  Solution sol;
  sol.printList(linked_list);
  ListNode *new_node = sol.removeNthFromEnd(linked_list, 2);
  sol.printList(new_node);
  return 0;
}
