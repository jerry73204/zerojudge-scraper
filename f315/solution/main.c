#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_N 100000

typedef struct {
    int value;
    int index;
} Pair;

// Function to merge two sorted subarrays and count inversions
long long merge(Pair arr[], Pair temp[], int left, int mid, int right) {
    int i = left;      // Index for left subarray
    int j = mid + 1;   // Index for right subarray
    int k = left;      // Index for merged array
    long long inv_count = 0;
    
    while (i <= mid && j <= right) {
        if (arr[i].value <= arr[j].value) {
            temp[k++] = arr[i++];
        } else {
            // If arr[i] > arr[j], then all elements from i to mid are greater than arr[j]
            inv_count += (mid - i + 1);
            temp[k++] = arr[j++];
        }
    }
    
    // Copy the remaining elements
    while (i <= mid) {
        temp[k++] = arr[i++];
    }
    
    while (j <= right) {
        temp[k++] = arr[j++];
    }
    
    // Copy back to original array
    for (i = left; i <= right; i++) {
        arr[i] = temp[i];
    }
    
    return inv_count;
}

// Merge sort function that counts inversions
long long mergeSort(Pair arr[], Pair temp[], int left, int right) {
    long long inv_count = 0;
    
    if (left < right) {
        int mid = left + (right - left) / 2;
        
        // Count inversions in left subarray
        inv_count += mergeSort(arr, temp, left, mid);
        
        // Count inversions in right subarray
        inv_count += mergeSort(arr, temp, mid + 1, right);
        
        // Count inversions during merge
        inv_count += merge(arr, temp, left, mid, right);
    }
    
    return inv_count;
}

int main() {
    int n;
    int nums[2 * MAX_N];
    Pair segments[MAX_N][2]; // To store segments where each number appears
    Pair arr[2 * MAX_N];
    Pair temp[2 * MAX_N];
    
    // Read input
    scanf("%d", &n);
    for (int i = 0; i < 2 * n; i++) {
        scanf("%d", &nums[i]);
    }
    
    // Initialize segment information
    for (int i = 1; i <= n; i++) {
        segments[i-1][0].index = -1;
        segments[i-1][0].value = -1;
        segments[i-1][1].index = -1;
        segments[i-1][1].value = -1;
    }
    
    // Record start and end indices for each number
    for (int i = 0; i < 2 * n; i++) {
        int num = nums[i];
        if (segments[num-1][0].index == -1) {
            segments[num-1][0].index = i;
            segments[num-1][0].value = num;
        } else {
            segments[num-1][1].index = i;
            segments[num-1][1].value = num;
        }
    }
    
    long long total_depression = 0;
    
    // For each number, count inversions between its two occurrences
    for (int i = 1; i <= n; i++) {
        int start = segments[i-1][0].index;
        int end = segments[i-1][1].index;
        int count = 0;
        
        // Create array of numbers between first and second occurrence
        int segment_size = 0;
        for (int j = start + 1; j < end; j++) {
            // Only count numbers less than i
            if (nums[j] < i) {
                arr[segment_size].value = nums[j];
                arr[segment_size].index = segment_size;
                segment_size++;
            }
        }
        
        // The depression value is simply the count of numbers smaller than i
        total_depression += segment_size;
    }
    
    printf("%lld", total_depression);
    
    return 0;
}