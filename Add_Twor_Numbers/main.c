#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

struct ListNode{
    int val;
    struct ListNode* next;
};

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2);

int main() {
    printf("Hello World!");
    return 0;
}

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode *result = malloc(sizeof(struct ListNode));
    struct ListNode *current = result; // Keep track of the current node
    int remainder = 0, sum;

    // Initialize the first node in the result list
    current->val = 0;
    current->next = NULL;

    while (l1 != NULL || l2 != NULL || remainder != 0) {
        sum = remainder;
        if (l1 != NULL) {
            sum += l1->val;
            l1 = l1->next;
        }
        if (l2 != NULL) {
            sum += l2->val;
            l2 = l2->next;
        }
        if (sum >= 10) {
            sum -= 10;
            remainder = 1;
        } else {
            remainder = 0;
        }
        current->val = sum;
        if (l1 != NULL || l2 != NULL || remainder != 0) {
            current->next = malloc(sizeof(struct ListNode));
            current = current->next;
            current->val = 0;
            current->next = NULL;
        }
    }

    return result;
}