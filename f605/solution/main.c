#include <stdio.h>
#include <stdlib.h>

// Function to find the maximum value in an array
int findMax(int arr[], int size) {
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

// Function to find the minimum value in an array
int findMin(int arr[], int size) {
    int min = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] < min) {
            min = arr[i];
        }
    }
    return min;
}

// Function to calculate the average of an array
int calcAverage(int arr[], int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum / size;
}

int main() {
    int n, d;
    scanf("%d %d", &n, &d);
    
    int count = 0;          // Number of products to buy
    int totalCost = 0;      // Total cost of all purchases
    
    // Process each product
    for (int i = 0; i < n; i++) {
        int prices[3];      // Prices for the past 3 days
        
        // Read the 3 prices for this product
        scanf("%d %d %d", &prices[0], &prices[1], &prices[2]);
        
        // Find max and min prices
        int maxPrice = findMax(prices, 3);
        int minPrice = findMin(prices, 3);
        
        // Check if price fluctuation is at least d
        if (maxPrice - minPrice >= d) {
            count++;
            totalCost += calcAverage(prices, 3);
        }
    }
    
    // Output result
    printf("%d %d\n", count, totalCost);
    
    return 0;
}