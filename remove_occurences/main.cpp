#include <iostream>
#include <string>

class Solution {
public:
  std::string removeOccurrences(std::string s, std::string part) {
    size_t pos = s.find(part);
    if (pos == std::string::npos) {
      return s;
    }
    s.erase(pos, part.size());
    return removeOccurrences(s, part);
  }
};

int main(void) {
  Solution sol;
  std::cout << sol.removeOccurrences("daabcbaabcbc", "abc") << std::endl;
  return 0;
}
