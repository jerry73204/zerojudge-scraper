#include <stdio.h>
#include <stdlib.h>

#define MAX_N 50000

// Comparison function for qsort
int compare(const void *a, const void *b) {
    return (*(int*)a - *(int*)b);
}

// Check if we can cover all points with at most K base stations with given diameter
int isPossible(int positions[], int n, int k, int diameter) {
    if (k <= 0) return 0;
    
    int stations = 1;  // Start with 1 station
    int currentPos = positions[0];  // Position of the first point
    
    for (int i = 1; i < n; i++) {
        // If the current point is beyond the reach of the current base station
        if (positions[i] - currentPos > diameter) {
            // Need to place a new base station
            stations++;
            // Position it at the current point (we'll place it optimally later)
            currentPos = positions[i];
            
            // If we exceed available stations, return false
            if (stations > k) {
                return 0;
            }
        }
    }
    
    return 1; // We can cover all points with at most k stations
}

int main() {
    int n, k;
    int positions[MAX_N];
    
    scanf("%d %d", &n, &k);
    
    for (int i = 0; i < n; i++) {
        scanf("%d", &positions[i]);
    }
    
    // Sort positions (they're not guaranteed to be sorted)
    qsort(positions, n, sizeof(int), compare);
    
    // Binary search for the minimum diameter
    int left = 0;
    int right = positions[n-1] - positions[0];  // Maximum possible diameter
    int result = right;
    
    while (left <= right) {
        int mid = left + (right - left) / 2;
        
        if (isPossible(positions, n, k, mid)) {
            // This diameter works, try a smaller one
            result = mid;
            right = mid - 1;
        } else {
            // This diameter is too small, try a larger one
            left = mid + 1;
        }
    }
    
    printf("%d\n", result);
    
    return 0;
}