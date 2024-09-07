#include <bits/stdc++.h>

using namespace std;

class Solution{
public:
    int lengthOfLongestSubstring(string &s){
    unordered_map<char, int> chars;
    int max_count = 0, beginID = 0;
        for (int i = 0; i < s.size(); ++i) {
            char c = s[i];
            if (chars.find(c) != chars.end() && chars[c] >= beginID) {
                beginID = chars[c]+1;
            }
            chars[c] = i;
            int count = i - beginID+1;
            if (count > max_count) {
                max_count = count;
            }
        }
        return max_count;
    }
};

int main() {
    string myStr = "abcbba";
    Solution solution;
    int lengthOfLongest = solution.lengthOfLongestSubstring(myStr);
    cout << "The highest length of a substring is "<< lengthOfLongest<<endl;
    return 0;
}
