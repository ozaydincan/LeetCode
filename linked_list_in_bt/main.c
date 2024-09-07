#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode* next;
};

struct TreeNode {
    int val;
    struct TreeNode* left;
    struct TreeNode* right;
};

struct ListNode* createListNode(int val) {
    struct ListNode* node = (struct ListNode*)malloc(sizeof(struct ListNode));
    node->val = val;
    node->next = NULL;
    return node;
}

struct ListNode* createLinkedList(int* values, int size) {
    if (size == 0) return NULL;
    struct ListNode* head = createListNode(values[0]);
    struct ListNode* current = head;
    for (int i = 1; i < size; ++i) {
        current->next = createListNode(values[i]);
        current = current->next;
    }
    return head;
}

struct TreeNode* createTreeNode(int val) {
    struct TreeNode* node = (struct TreeNode*)malloc(sizeof(struct TreeNode));
    node->val = val;
    node->left = NULL;
    node->right = NULL;
    return node;
}

struct TreeNode* createBinaryTree(int* values, int size) {
    if (size == 0) return NULL;
    struct TreeNode** nodes = (struct TreeNode**)malloc(size * sizeof(struct TreeNode*));
    for (int i = 0; i < size; ++i) {
        if (values[i] != -1) {
            nodes[i] = createTreeNode(values[i]);
        } else {
            nodes[i] = NULL;
        }
    }
    for (int i = 0; i < size; ++i) {
        if (nodes[i]) {
            int leftIndex = 2 * i + 1;
            int rightIndex = 2 * i + 2;
            if (leftIndex < size) nodes[i]->left = nodes[leftIndex];
            if (rightIndex < size) nodes[i]->right = nodes[rightIndex];
        }
    }
    struct TreeNode* root = nodes[0];
    free(nodes);
    return root;
}

bool dfs(struct ListNode* head, struct TreeNode* root) {
    if (!head) return true;
    if (!root) return false;
    if (root->val != head->val) return false;
    return dfs(head->next, root->left) || dfs(head->next, root->right);
}

bool isSubPath(struct ListNode* head, struct TreeNode* root) {
    if (!root) return false;
    if (dfs(head, root)) return true;
    return isSubPath(head, root->left) || isSubPath(head, root->right);
}

int main(void) {
    int list_values[] = {4, 2, 8}; 
    int tree_values[] = {1, 4, 4, -1, 2, 2, -1, 1, -1, 6, 8, -1, -1, -1, -1, 1, 3};

    struct ListNode* listHead = createLinkedList(list_values, sizeof(list_values) / sizeof(list_values[0]));
    struct TreeNode* treeRoot = createBinaryTree(tree_values, sizeof(tree_values) / sizeof(tree_values[0]));
    
    int containsLinkedList = isSubPath(listHead, treeRoot);

    switch (containsLinkedList){
        case 1:
            printf("Contains a linked list\n");
            break;
        case 0:
            printf("Doesn't contains a linked list\n");
            break;
    }

    return 0;
}

