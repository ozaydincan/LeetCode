#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
  int carFleet(int target, vector<int> &position, vector<int> &speed) {
    int n = position.size();
    vector<vector<int>> cars;

    for (int i = 0; i < n; i++) {
      cars.push_back({position[i], speed[i]});
    }

    sort(cars.rbegin(), cars.rend());

    vector<double> fleets;

    for (vector<int> &car : cars) {
      double time = (double)(target - car[0]) / car[1];
      fleets.push_back(time);
      int fleet_n = fleets.size();
      if (fleet_n >= 2 && fleets[fleet_n - 1] <= fleets[fleet_n - 2]) {
        fleets.pop_back();
      }
    }

    return fleets.size();
  }
};

int main(int argc, char *argv[]) {
  Solution sol;
  int target = 10;
  vector<int> pos = {1, 3};
  vector<int> speed = {3, 2};
  cout << sol.carFleet(target, pos, speed) << " fleets" << '\n';
  return 0;
}
