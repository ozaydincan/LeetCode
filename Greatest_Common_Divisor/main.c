#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode* next;
};


int greatestCommonDivisor(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

struct ListNode* addNode(int val){
    struct ListNode* node = (struct ListNode*) malloc(sizeof(struct ListNode));
    if(!node){
        perror("Node malloc error");
        exit(EXIT_FAILURE);
    }
    node->val = val;
    node->next = NULL;
    return node;
}


struct ListNode* insertGreatestCommonDivisors(struct ListNode* head){
    if(!head){
        perror("Empty list head!");
        exit(EXIT_FAILURE);
    }
    if(!head->next){
        return head;
    }
    struct ListNode* curr = head->next;
    struct ListNode* prev = head;
    while(curr){
        int gcd = greatestCommonDivisor(curr->val, prev->val);
        struct ListNode* inter = addNode(gcd);
        prev->next = inter;
        inter->next = curr;
        prev = curr;
        curr = curr->next;
    }
    return head;
}
int main(void){
    struct ListNode* head = addNode(18);
    head->next = addNode(6);
    head->next->next = addNode(10);
    head->next->next->next = addNode(3);    
    while(head){
        printf("%d ", head->val);
    }
    return 0;
}
