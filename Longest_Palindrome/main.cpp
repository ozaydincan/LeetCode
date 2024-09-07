#include <bits/stdc++.h>
using namespace std;

// Function to print a substring str[low..high]
void printSubStr(string str, int low, int high)
{
    for (int i = low; i <= high; ++i)
        cout << str[i];
}

// Function to find the longest palindromic substring
class Solution{
    public:
    string longestPalSubstr(string s){
    int n = s.length();
    int start = 0, end = 1;
    int low, hi;

    // Traverse the input string
    for (int i = 0; i < n; i++) {

        // Find longest palindromic substring of even size
        low = i - 1;
        hi = i;

        // Expand substring while it is palindrome
        // and in bounds
        while (low >= 0 && hi < n && s[low] == s[hi]) {

            // Update maximum length and starting index
            if (hi - low + 1 > end) {
                start = low;
                end = hi - low + 1;
            }
            low--;
            hi++;
        }

        // Find longest palindromic substring of odd size
        low = i - 1;
        hi = i + 1;

        // Expand substring while it is palindrome
        // and in bounds
        while (low >= 0 && hi < n && s[low] == s[hi]) {

            // Update maximum length and starting index
            if (hi - low + 1 > end) {
                start = low;
                end = hi - low + 1;
              
            }
               low--;
            hi++;
        }
    }

    // Print the longest palindromic substring
    cout << "Longest palindrome substring is: ";


    // Return output length
    return s.substr(start, end);
    }

};

int main()
{
    string str = "forgeeksskeegfor";
    Solution solution;
    // Function call
    string sub_str = solution.longestPalSubstr(str);
    cout  << sub_str;
    return 0;
}
