#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>

class Solution{
    private:
    std::vector<int> values(std::unordered_map<int, int> &u_mp){
        std::vector<int> values;
        for(const auto&i: u_mp){
            values.push_back(i.second);
        }
        return values;
    }
    public:
        int maxFrequencyElements(std::vector<int>& nums){
            std::unordered_map<int, int> freqs;
            for (const int num: nums){
                freqs[num]++;
            }
            std::vector<int> u_map_values = values(freqs);            
            int max_value = *std::max_element(u_map_values.begin(),u_map_values.end());
            int count = 0, key_count = 0;
            for (const auto& [key, value]: freqs){
                if(value == max_value){
                    count++;
                    key_count++;
                }
            }
            return max_value * count;
        }
};


int main(){
    Solution solution;
    std::vector <int> nums = {1,2,3,4,5};
    std::cout << solution.maxFrequencyElements(nums);
    return 0;
}