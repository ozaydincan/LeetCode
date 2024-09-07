#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    int countSeniors(vector<std::string>& details) {
    int count = 0;
    for(string str: details){
        size_t pos = str.find('M') != string::npos ?
        str.find('M'): str.find('F');
        int age = stoi(str.substr(pos+1, 2));
        if(age > 60){
            count++;
        } 
    }     
    return count;   
    }
};

int main(){
    vector<string> details = {"5612624052M0130","5378802576M6424","5447619845F0171","2941701174O9078"};
    Solution solution;
    cout << "There are "<< solution.countSeniors(details)<<" senior passengers";
    return 0;
}