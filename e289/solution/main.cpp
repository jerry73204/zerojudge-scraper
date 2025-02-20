#include <cstdint>
#include <deque>
#include <iostream>
#include <unordered_map>
#include <vector>

int main() {
    // Disable synchronizatoin with C buffers to speed up.
    std::ios::sync_with_stdio(false);

    // Read m and n.
    int m, n;
    std::cin >> m >> n;
    // Check the case when m is larger then n.
    // In this case, the answer is zero.
    if (m > n) {
        std::cout << 0 << std::endl;
        return 0;
    }

    // Create the sliding window
    std::deque<uint64_t> window;

    // Create an unordered map to maintain the counts of distinct
    // numbers within the sliding window.
    std::unordered_map<uint64_t, int> counts;

    // Initialize the count to zero.
    int count = 0;
    int round = 0;

    // Populate first n numbers to the counts map.
    while (round < m) {
        uint64_t val;
        std::cin >> val;
        window.push_back(val);
        counts[val] += 1;
        round += 1;
    }

    // Add one count if the first m numbers are distinct.
    if (counts.size() == m) {
        count += 1;
    }

    // In each loop, drop the leftomst number and add one rightmost
    // number. Decrease one count on the leftmost number and increase
    // one count at the rightmost number.
    while (round < n) {
        // Get the leftmost number in the window.
        uint64_t lval = window.front();
        window.pop_front();

        // Decrease the count on the leftmost number.
        auto dit = counts.find(lval);
        dit->second -= 1;

        // If the count of the leftmost number decreases to zero,
        // remove it from the count map.
        if (dit->second == 0) {
            counts.erase(dit);
        }

        // Read a new number and put it on the rightmost position in
        // the window.
        uint64_t rval;
        std::cin >> rval;
        window.push_back(rval);

        // Increase one count on the rightmost number.
        counts[rval] += 1;

        // Check if there are m distinct numbers in the count map.
        if (counts.size() == m) {
            count += 1;
        }

        // Add one round.
        round += 1;
    }

    // Print the answer.
    std::cout << count << std::endl;
    return 0;
}
