#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MAX_N 200000

typedef struct {
    int x, y;
} Point;

// Compare points by x-coordinate (ascending), and if x is same, by y-coordinate (ascending)
int comparePoints(const void *a, const void *b) {
    Point *p1 = (Point*)a;
    Point *p2 = (Point*)b;
    
    if (p1->x != p2->x) {
        return p1->x - p2->x;
    }
    return p1->y - p2->y;
}

// Binary search to find the smallest element in tails that is greater than or equal to key
int binarySearch(int tails[], int len, int key) {
    int left = 0, right = len - 1;
    
    while (left <= right) {
        int mid = left + (right - left) / 2;
        
        if (tails[mid] < key) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    return left;
}

// Function to find the length of the longest increasing subsequence (LIS)
// using the efficient O(n log n) algorithm
int findLIS(Point points[], int n) {
    if (n == 0) return 0;
    
    // After sorting by x, we only need to consider the LIS for y coordinates
    // First, filter out points with the same x but keeping only the one with smallest y
    int filtered_count = 0;
    Point filtered[MAX_N];
    
    for (int i = 0; i < n; i++) {
        // Skip points with the same x-coordinate as the previous one
        // (except the first one)
        if (i == 0 || points[i].x != points[i-1].x) {
            filtered[filtered_count++] = points[i];
        } else if (points[i].y < filtered[filtered_count-1].y) {
            // If same x but smaller y, replace the previous one
            filtered[filtered_count-1] = points[i];
        }
    }
    
    // Array to store the tails of the LIS
    int tails[MAX_N];
    int len = 0; // Length of LIS
    
    // Process points with filtered duplicates
    for (int i = 0; i < filtered_count; i++) {
        int y = filtered[i].y;
        
        // Case 1: y is greater than the last element in tails
        if (len == 0 || y > tails[len-1]) {
            tails[len++] = y;
        } 
        // Case 2: y is smaller than or equal to the last element
        // Find the smallest element >= y using binary search
        else {
            int pos = binarySearch(tails, len, y);
            tails[pos] = y;
        }
    }
    
    return len;
}

int main() {
    int n;
    Point points[MAX_N];
    
    // Read input
    scanf("%d", &n);
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &points[i].x, &points[i].y);
    }
    
    // Sort points by x-coordinate (and by y if x is the same)
    qsort(points, n, sizeof(Point), comparePoints);
    
    // Find the LIS based on y-coordinates
    int result = findLIS(points, n);
    
    // Output the result
    printf("%d", result);
    
    return 0;
}