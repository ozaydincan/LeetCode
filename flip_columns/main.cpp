#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
  int maxEqualRowsAfterFlips(vector<vector<int> > &matrix) {
    unordered_map<string, int> freqs;

    for (const auto &row : matrix) {
      string pattern;
      for (int j = 0; j < row.size(); j++) {
        pattern += (row[j] == row[0]) ? '0' : '1';
      }
      freqs[pattern]++;
    }

    int maxFrequency = 0;
    for (const auto &entry : freqs) {
      maxFrequency = max(maxFrequency, entry.second);
    }
    return maxFrequency;
  }
};
