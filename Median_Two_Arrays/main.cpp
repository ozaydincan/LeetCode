#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>


class Solution{
    private:
    double findMeanSortedArrays(std::vector<int> &nums1, std::vector<int> &nums2){
        std::unordered_map<int, int> u_map;
        nums1.insert(nums1.end(), nums2.begin(), nums2.end());
        for(const auto& num: nums1){
            u_map[num]++;
        }
        double total_sum = 0;
        for (const auto [key, value]: u_map){
            total_sum += key*value;
        }
        double median = total_sum/nums1.size();
        return median;
    }
    public:
    double findMedianSortedArrays(std::vector<int>& nums1, std::vector<int>& nums2) {
        std::vector<int> merged(nums1.size()+nums2.size());
        std::merge(nums1.begin(), nums1.end(), nums2.begin(), nums2.end(), merged.begin());
        double median = 0;
        if(merged.size()%2== 0){
            double id1 = (merged.size()/2)-1;
            double med1 = merged[id1];
            double id2 =(merged.size()/2);
            double med2 = merged[id2];
            median = (med1 + med2)/2; 
        }else{
            double id = (merged.size()-1)/2;
            median = merged[id];
        }
        return median;

    }
};

int main(){
    Solution solution;
    std::vector<int> nums1 = {1,2};
    std::vector<int> nums2 = {3,4};
    double median = solution.findMedianSortedArrays(nums1, nums2);
    std::cout << median;
    return 0;
}