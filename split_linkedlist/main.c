#include <cstdio>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

void fillNode(struct ListNode** head, size_t n, int arr[]) {
    struct ListNode* tail = NULL;
    
    for (int i = 0; i < n; i++) {
        struct ListNode* newNode = (struct ListNode*)malloc(sizeof(struct ListNode));
        newNode->val = arr[i];
        
        if (*head == NULL) {
            *head = newNode;
        } else {
            tail->next = newNode;
        }
        tail = newNode;
    }
}

void printList(struct ListNode* head) {
    while (head != NULL) {
        printf("%d ", head->val);
        head = head->next;
    }
    printf("\n");
}

struct ListNode** splitListToParts(struct ListNode* head, int k, int* returnSize) {
    *returnSize = k;

    struct ListNode** newList = (struct ListNode**)malloc(k * sizeof(struct ListNode*));

    int list_len = 0;
    struct ListNode* temp = head;
    while (temp != NULL) {
        list_len++;
        temp = temp->next;
    }

    int chunk = list_len / k;
    int remainder = list_len % k;

    for (int i = 0; i < k; ++i) {
        newList[i] = head;
        int part_size = chunk + (i < remainder ? 1 : 0);

        for (int j = 0; j < part_size - 1 && head != NULL; ++j) {
            head = head->next;
        }

        if (head != NULL) {
            struct ListNode* next = head->next;
            head->next = NULL;
            head = next;
        }
    }

    return newList;
}


int main(void) {
    struct ListNode* head = NULL;
    int inputs[] = {1, 2, 3};
    fillNode(&head, sizeof(inputs)/sizeof(inputs[0]), inputs);
    printList(head);

    struct ListNode* temp;
    while (head != NULL) {
        temp = head;
        head = head->next;
        free(temp);
    }

    return EXIT_SUCCESS;
}

