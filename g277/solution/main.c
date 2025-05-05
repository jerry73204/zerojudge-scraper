#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

// Function to find the lucky number in the given interval
int findLuckyNumber(int arr[], int L, int R) {
    // Base case: if interval has only one element, it's the lucky number
    if (L == R) {
        return arr[L];
    }
    
    // Base case: if interval is empty, return a special value
    if (L > R) {
        return -1; // This should not happen with proper input
    }
    
    // Find the position of the minimum number in the interval
    int minVal = INT_MAX;
    int minPos = -1;
    for (int i = L; i <= R; i++) {
        if (arr[i] < minVal) {
            minVal = arr[i];
            minPos = i;
        }
    }
    
    // Calculate sum of left interval [L, minPos-1]
    int leftSum = 0;
    if (minPos > L) { // Ensure left interval is not empty
        for (int i = L; i < minPos; i++) {
            leftSum += arr[i];
        }
    }
    
    // Calculate sum of right interval [minPos+1, R]
    int rightSum = 0;
    if (minPos < R) { // Ensure right interval is not empty
        for (int i = minPos + 1; i <= R; i++) {
            rightSum += arr[i];
        }
    }
    
    // Determine which interval contains the lucky number
    if (leftSum > rightSum) {
        // Lucky number is in the left interval
        if (minPos > L) { // Ensure left interval is not empty
            return findLuckyNumber(arr, L, minPos - 1);
        } else {
            // This should not happen with properly formed intervals
            return -1;
        }
    } else {
        // Lucky number is in the right interval (includes case where leftSum == rightSum)
        if (minPos < R) { // Ensure right interval is not empty
            return findLuckyNumber(arr, minPos + 1, R);
        } else {
            // This can happen if rightSum = 0 (empty right interval) and leftSum = 0
            return -1;
        }
    }
}

int main() {
    int n;
    scanf("%d", &n);
    
    int *arr = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }
    
    // Handle edge cases
    if (n == 1) {
        printf("%d\n", arr[0]);
    } else {
        // Find the lucky number
        int luckyNumber = findLuckyNumber(arr, 0, n - 1);
        printf("%d\n", luckyNumber);
    }
    
    free(arr);
    return 0;
}