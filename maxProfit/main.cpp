#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
  int maxProfit(vector<int> &prices) {
    int left = 0;
    int right = 1;
    int profit = 0;
    while (right < prices.size()) {
      if (prices[left] < prices[right]) {
        int curr_profit = prices[right] - prices[left];
        profit = (profit > curr_profit) ? profit : curr_profit;
      } else {
        left = right;
      }
      right++;
    }

    return profit;
  }
};

int main(int argc, char *argv[]) {
  vector<int> prices = {10, 8, 7, 5, 2};
  Solution sol;
  cout << "The max profit that can be obtained is " << sol.maxProfit(prices)
       << '\n';
  return 0;
}
