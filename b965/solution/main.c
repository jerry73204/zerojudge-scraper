#include <stdio.h>
#include <stdlib.h>

#define MAX_SIZE 10

// Function to flip a matrix (swap rows)
void flip(int matrix[MAX_SIZE][MAX_SIZE], int rows, int cols) {
    for (int i = 0; i < rows / 2; i++) {
        for (int j = 0; j < cols; j++) {
            int temp = matrix[i][j];
            matrix[i][j] = matrix[rows - 1 - i][j];
            matrix[rows - 1 - i][j] = temp;
        }
    }
}

// Function to rotate a matrix 90 degrees clockwise
void rotate(int matrix[MAX_SIZE][MAX_SIZE], int *rows, int *cols) {
    int temp[MAX_SIZE][MAX_SIZE];
    
    // Copy the rotated matrix to temp
    for (int i = 0; i < *rows; i++) {
        for (int j = 0; j < *cols; j++) {
            temp[j][*rows - 1 - i] = matrix[i][j];
        }
    }
    
    // Swap rows and columns
    int temp_rows = *rows;
    *rows = *cols;
    *cols = temp_rows;
    
    // Copy back from temp
    for (int i = 0; i < *rows; i++) {
        for (int j = 0; j < *cols; j++) {
            matrix[i][j] = temp[i][j];
        }
    }
}

// Function to print the matrix
void print_matrix(int matrix[MAX_SIZE][MAX_SIZE], int rows, int cols) {
    printf("%d %d\n", rows, cols);
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            printf("%d", matrix[i][j]);
            if (j < cols - 1) {
                printf(" ");
            }
        }
        if (i < rows - 1) {
            printf("\n");
        }
    }
}

int main() {
    int R, C, M;
    int matrix[MAX_SIZE][MAX_SIZE];
    int operations[MAX_SIZE];
    
    // Read input
    scanf("%d %d %d", &R, &C, &M);
    
    // Read matrix B
    for (int i = 0; i < R; i++) {
        for (int j = 0; j < C; j++) {
            scanf("%d", &matrix[i][j]);
        }
    }
    
    // Read operations
    for (int i = 0; i < M; i++) {
        scanf("%d", &operations[i]);
    }
    
    // Reverse the operations to get matrix A
    for (int i = M - 1; i >= 0; i--) {
        if (operations[i] == 0) {
            // Reverse of clockwise rotation is counter-clockwise rotation
            // = 3 clockwise rotations
            rotate(matrix, &R, &C);
            rotate(matrix, &R, &C);
            rotate(matrix, &R, &C);
        } else {
            // Reverse of flip is flip again
            flip(matrix, R, C);
        }
    }
    
    // Print matrix A
    print_matrix(matrix, R, C);
    
    return 0;
}