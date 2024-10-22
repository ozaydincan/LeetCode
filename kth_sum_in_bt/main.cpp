#include <algorithm>
#include <cstddef>
#include <iostream>
#include <queue>
#include <vector>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution {
public:
  long long kthLargestLevelSum(TreeNode *root, int k) {
    if (!root) {
      return -1;
    }
    std::vector<long long> sums;
    std::queue<TreeNode *> q;
    q.push(root);

    while (!q.empty()) {
      int n = q.size();
      long long level_sum = 0;
      for (size_t i = 0; i < n; i++) {
        TreeNode *node = q.front();
        q.pop();
        level_sum += node->val;

        if (node->left != nullptr) {
          q.push(node->left);
        }
        if (node->right != nullptr) {
          q.push(node->right);
        }
      }
      sums.push_back(level_sum);
    }
    std::sort(sums.begin(), sums.end());
    if (k > sums.size()) {
      return -1;
    }

    return sums[sums.size() - k];
  }
};

int main(int argc, char *argv[]) {

  TreeNode *root = new TreeNode(1);
  root->left = new TreeNode(2);
  root->right = new TreeNode(3);
  root->left->left = new TreeNode(4);
  root->left->right = new TreeNode(5);
  root->right->left = new TreeNode(6);
  root->right->right = new TreeNode(7);

  Solution sol;
  int k = 2;
  std::cout << "The " << k
            << "-th largest level sum is: " << sol.kthLargestLevelSum(root, k)
            << std::endl;

  return 0;
}
