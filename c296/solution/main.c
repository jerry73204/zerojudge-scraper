#include <stdio.h>
#include <stdlib.h>

// Solves the "timed K-bomb" problem (modified Josephus problem)
// N people in a circle (numbered 1 to N)
// Pass bomb, every M-th person is eliminated
// After K eliminations, the next person is the "lucky one"
int findLuckyPerson(int n, int m, int k) {
    if (n == 1) return 1;
    
    // Use a circular linked list to represent people in the circle
    typedef struct Node {
        int value;           // Person number
        struct Node* next;   // Next person in circle
    } Node;
    
    // Create circular linked list of n people
    Node* head = malloc(sizeof(Node));
    head->value = 1;
    
    Node* current = head;
    for (int i = 2; i <= n; i++) {
        Node* newNode = malloc(sizeof(Node));
        newNode->value = i;
        current->next = newNode;
        current = newNode;
    }
    current->next = head;  // Make the list circular
    
    // Start eliminating people
    current = head;
    Node* prev = current;
    while (prev->next != current) {
        prev = prev->next;
    }
    
    int eliminated = 0;
    while (eliminated < k) {
        // Find the M-th person to eliminate
        for (int i = 1; i < m; i++) {
            prev = current;
            current = current->next;
        }
        
        // Eliminate current person
        prev->next = current->next;
        Node* temp = current;
        current = current->next;  // Move to next person
        free(temp);
        eliminated++;
    }
    
    // Return the lucky person (person after the last eliminated one)
    int luckyPerson = current->value;
    
    // Clean up memory (free remaining nodes)
    Node* start = current;
    current = current->next;
    while (current != start) {
        Node* temp = current;
        current = current->next;
        free(temp);
    }
    free(start);
    
    return luckyPerson;
}

// Optimized solution for large N values using math formula (O(k) time complexity)
int findLuckyPersonOptimized(int n, int m, int k) {
    if (k >= n) {
        k = n - 1;  // Can't eliminate more than n-1 people
    }
    
    // Start with a circle of size n
    int position = 0;  // 0-indexed
    
    // Perform k eliminations
    for (int size = n; size > n - k; size--) {
        position = (position + m - 1) % size;
    }
    
    // Calculate the lucky person (the next person after the last elimination)
    position = (position + 1) % (n - k);
    if (position == 0) position = n - k;
    
    // Convert to 1-indexed result
    return position;
}

int main() {
    int n, m, k;
    
    // Read input
    scanf("%d %d %d", &n, &m, &k);
    
    // Find the lucky person
    int result;
    if (n <= 10000) {
        // For smaller inputs, use the linked list solution
        result = findLuckyPerson(n, m, k);
    } else {
        // For larger inputs, use the optimized solution
        result = findLuckyPersonOptimized(n, m, k);
    }
    
    // Output the result
    printf("%d\n", result);
    
    return 0;
}