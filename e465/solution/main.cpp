#include <climits>
#include <iostream>
#include <vector>

int main() {
  std::ios_base::sync_with_stdio(false);
 
  // Read m, s, n
  int m, s, n;
  std::cin >> m >> s >> n;


  // Read numbers
  std::vector<int> nums;
  nums.reserve(n);

  for(int &val: nums) {
    std::cin >> val;
  }

  // Compute minimum sums at each number
  std::vector<int> mins;
  nums.reserve(n);

  mins[n - 1] = mins[n - 1];
  for (int i = n - 2; i >= 0; i -= 1) {
    mins[i] = mins[i + 1] + nums[i];
  }

  for(int &val: mins) {
    val = std::max(s - val, 0);
  }

  int min_sum = INT_MAX;
  
  return 0;
}
