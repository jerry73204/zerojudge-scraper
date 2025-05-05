#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_N 100
#define MAX_M 100
#define MAX_K 500

typedef struct {
    int r, c;   // Current position
    int s, t;   // Movement vector
    bool alive; // Whether the demon is still on the board
} Demon;

int main() {
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);
    
    Demon demons[MAX_K];
    bool hasBomb[MAX_N][MAX_M] = {false}; // Tracks where bombs are placed
    bool hasDemon[MAX_N][MAX_M] = {false}; // Tracks where demons are currently
    
    // Read demon data
    for (int i = 0; i < k; i++) {
        scanf("%d %d %d %d", &demons[i].r, &demons[i].c, &demons[i].s, &demons[i].t);
        demons[i].alive = true;
        hasDemon[demons[i].r][demons[i].c] = true;
    }
    
    bool demonsRemaining = true;
    while (demonsRemaining) {
        // Place bombs where demons are currently located
        for (int i = 0; i < k; i++) {
            if (demons[i].alive) {
                hasBomb[demons[i].r][demons[i].c] = true;
            }
        }
        
        // Clear the current demon positions
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                hasDemon[i][j] = false;
            }
        }
        
        // Move demons to new positions
        for (int i = 0; i < k; i++) {
            if (demons[i].alive) {
                demons[i].r += demons[i].s;
                demons[i].c += demons[i].t;
                
                // Check if the demon moved out of bounds
                if (demons[i].r < 0 || demons[i].r >= n || demons[i].c < 0 || demons[i].c >= m) {
                    demons[i].alive = false;
                } else {
                    // Mark the new position as having a demon
                    hasDemon[demons[i].r][demons[i].c] = true;
                }
            }
        }
        
        // Check for bombs that explode and kill demons
        for (int i = 0; i < k; i++) {
            if (demons[i].alive && hasBomb[demons[i].r][demons[i].c]) {
                // Demon stepped on a bomb
                for (int j = 0; j < k; j++) {
                    if (demons[j].alive && demons[j].r == demons[i].r && demons[j].c == demons[i].c) {
                        demons[j].alive = false;
                    }
                }
                // Remove the bomb
                hasBomb[demons[i].r][demons[i].c] = false;
            }
        }
        
        // Check if any demons remain
        demonsRemaining = false;
        for (int i = 0; i < k; i++) {
            if (demons[i].alive) {
                demonsRemaining = true;
                break;
            }
        }
    }
    
    // Count the remaining bombs
    int bombCount = 0;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (hasBomb[i][j]) {
                bombCount++;
            }
        }
    }
    
    printf("%d\n", bombCount);
    
    return 0;
}