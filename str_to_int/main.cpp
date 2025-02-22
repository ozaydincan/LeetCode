#include <limits>
#include <string>

using namespace std;

class Solution {
public:
  int myAtoi(string s) {
    int i = 0, n = s.size(), sign = 1;
    long num = 0;

    while (i < n && s[i] == ' ') {
      i++;
    }

    if (i < n && (s[i] == '+' || s[i] == '-')) {
      sign = (s[i] == '-') ? -1 : 1;
      i++;
    }

    while (i < n && isdigit(s[i])) {
      num = num * 10 + (s[i] - '0');

      if (sign == 1 && num > numeric_limits<int>::max()) {
        return numeric_limits<int>::max();
      }
      if (sign == -1 && -num < numeric_limits<int>::min()) {
        return numeric_limits<int>::min();
      }

      i++;
    }

    return static_cast<int>(num * sign);
  }
};

