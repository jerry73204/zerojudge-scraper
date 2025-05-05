#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))

int main() {
    int n;
    scanf("%d", &n);
    
    // Read fence heights
    int heights[n];
    for (int i = 0; i < n; i++) {
        scanf("%d", &heights[i]);
    }
    
    int totalCost = 0;
    
    // Process each broken fence
    for (int i = 0; i < n; i++) {
        if (heights[i] == 0) { // Broken fence
            int leftHeight = INT_MAX;
            int rightHeight = INT_MAX;
            
            // Find the height of the closest non-broken fence to the left
            for (int j = i - 1; j >= 0; j--) {
                if (heights[j] > 0) {
                    leftHeight = heights[j];
                    break;
                }
            }
            
            // Find the height of the closest non-broken fence to the right
            for (int j = i + 1; j < n; j++) {
                if (heights[j] > 0) {
                    rightHeight = heights[j];
                    break;
                }
            }
            
            // If no fence found on one side, use the other side's height
            if (leftHeight == INT_MAX && rightHeight != INT_MAX) {
                leftHeight = rightHeight;
            } else if (rightHeight == INT_MAX && leftHeight != INT_MAX) {
                rightHeight = leftHeight;
            }
            
            // Calculate the new height of the fence (min of left and right)
            int newHeight = MIN(leftHeight, rightHeight);
            
            // Add the cost
            totalCost += newHeight;
        }
    }
    
    printf("%d\n", totalCost);
    
    return 0;
}