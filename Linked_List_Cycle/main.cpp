#include <bits/stdc++.h>

struct ListNode{
    int val;
    ListNode *next;
    ListNode (int x): val(x), next(nullptr){}
};

using namespace std;
class Solution{
    public:
    bool hasCycle(ListNode *head){
        unordered_map<ListNode*, int> linkedMap;
        while (head->next != nullptr){
            if(!linkedMap[head]){
                linkedMap[head]++;
            }else{
                return true;
            }
            head = head->next;
        }
        return false;
        }
};

ListNode* createLinkedList(const std::vector<int>& values) {
    if (values.empty()) return nullptr;

    ListNode* head = new ListNode(values[0]);
    ListNode* current = head;

    for (size_t i = 1; i < values.size(); ++i) {
        current->next = new ListNode(values[i]);
        current = current->next;
    }

    return head;
}

int main(){
    vector<int> values = {3,2,0,4};
    ListNode* ll = createLinkedList(values);
    Solution solution;
    return 0; 
}