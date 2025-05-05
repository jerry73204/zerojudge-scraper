#include <stdio.h>
#include <stdlib.h>

#define MAX_N 200000

typedef struct {
    int position;  // Position of the cut
    int order;     // Order of the cut (1 to n)
} Cut;

// Comparison function for sorting cuts by order
int compareCuts(const void* a, const void* b) {
    return ((Cut*)a)->order - ((Cut*)b)->order;
}

// Function to find which segment contains a given position
int findSegment(int position, int segments[][2], int segment_count) {
    for (int i = 0; i < segment_count; i++) {
        if (position >= segments[i][0] && position <= segments[i][1]) {
            return i;
        }
    }
    return -1; // Not found (shouldn't happen)
}

int main() {
    int n, L;
    Cut cuts[MAX_N];
    int segments[MAX_N+1][2]; // Each segment has [start, end]
    int segment_count = 1;    // Start with one segment
    
    // Read input
    scanf("%d %d", &n, &L);
    
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &cuts[i].position, &cuts[i].order);
    }
    
    // Sort cuts by order
    qsort(cuts, n, sizeof(Cut), compareCuts);
    
    // Initialize the first segment
    segments[0][0] = 0;
    segments[0][1] = L;
    
    long long total_cost = 0;
    
    // Process each cut
    for (int i = 0; i < n; i++) {
        int position = cuts[i].position;
        
        // Find which segment contains this position
        int segment_idx = findSegment(position, segments, segment_count);
        
        if (segment_idx != -1) {
            int start = segments[segment_idx][0];
            int end = segments[segment_idx][1];
            
            // Calculate the cost (length of the current segment)
            total_cost += (end - start);
            
            // Create two new segments
            segments[segment_idx][1] = position;         // Update the current segment
            segments[segment_count][0] = position;       // Add a new segment
            segments[segment_count][1] = end;
            segment_count++;
        }
    }
    
    // Output the total cost
    printf("%lld", total_cost);
    
    return 0;
}