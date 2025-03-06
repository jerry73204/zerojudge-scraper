/*
  Submission ID: 15605948
  AC (33ms, 1.3MB)
*/

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct grid_t {
    int *buf;
    int ncols;
    int ncells;
} Grid;

void grid_init(Grid *grid, int nrows, int ncols) {
    int ncells = nrows * ncols;

    grid->buf = (int *)malloc(sizeof(int) * ncells);
    assert(grid->buf != NULL);

    grid->ncols = ncols;
    grid->ncells = ncells;
}

void grid_destroy(Grid *grid) { free(grid->buf); }

int *grid_ptr(Grid *grid, int row, int col) {
    if (row < 0 || col < 0 || col >= grid->ncols) {
        return NULL;
    }

    int nth = row * grid->ncols + col;
    if (nth >= grid->ncells) {
        return NULL;
    }

    return &grid->buf[nth];
}

int grid_get(Grid *grid, int row, int col) {
    int *ptr = grid_ptr(grid, row, col);
    assert(ptr != NULL);
    return *ptr;
}

void sort2(int *a, int *b) {
    if (*a > *b) {
        int tmp = *a;
        *a = *b;
        *b = tmp;
    }
}

int max(int a, int b) {
    if (a >= b) {
        return a;
    } else {
        return b;
    }
}

void swap(int **a, int **b) {
    int *tmp = *a;
    *a = *b;
    *b = tmp;
}

void fill_prefix_sums(int *vals, int *sums, int n) {
    sums[0] = 0;

    for (int i = 0; i < n; i += 1) {
        sums[i + 1] = sums[i] + vals[i];
    }
}

int range_sum(int *sums, int i, int j) {
    sort2(&i, &j);
    return sums[j + 1] - sums[i];
}

void print_row(int *row, int n) {
    int *end = row + n;
    for (int *ptr = row; ptr != end; ptr += 1) {
        printf("% 5d", *ptr);
    }
    printf("\n");
}

int main() {
    int m, n;
    scanf("%d %d", &m, &n);

    Grid vals;
    grid_init(&vals, m, n);

    int *end = vals.buf + vals.ncells;
    for (int *ptr = vals.buf; ptr != end; ptr += 1) {
        scanf("%d", ptr);
    }

    int *prefix_sums = (int *)malloc(sizeof(int) * (n + 1));
    assert(prefix_sums != NULL);

    int *prev = (int *)malloc(sizeof(int) * n);
    assert(prev != NULL);

    int *curr = (int *)malloc(sizeof(int) * n);
    assert(curr != NULL);

    // Set the current buffer to zero
    memset(curr, 0, sizeof(int) * n);

    for (int r = 0; r < m; r += 1) {
        // Swap the current and previous buffer
        swap(&prev, &curr);

        // Get the pointer to the rth row in the grid.
        int *row = grid_ptr(&vals, r, 0);

        // Fill the array of sums
        fill_prefix_sums(row, prefix_sums, n);
        /* print_row(row, n); */
        /* print_row(prefix_sums + 1, n); */
        int row_sum = prefix_sums[n];

        int acc_0_to_maxc = INT_MIN;
        for (int c = n - 1; c >= 0; c -= 1) {
            int sum_0_to_c = prefix_sums[c + 1];
            int acc_0_to_c = prev[c] + sum_0_to_c;
            acc_0_to_maxc = max(acc_0_to_maxc, acc_0_to_c);

            int sum_0_to_c_sub_1 = prefix_sums[c];
            int acc_c_to_maxc = acc_0_to_maxc - sum_0_to_c_sub_1;
            curr[c] = acc_c_to_maxc;
        }
        /* print_row(curr, n); */

        int acc_maxc_to_end = INT_MIN;
        for (int c = 0; c < n; c += 1) {
            int sum_c_to_end = row_sum - prefix_sums[c];
            int acc_c_to_end = prev[c] + sum_c_to_end;
            acc_maxc_to_end = max(acc_maxc_to_end, acc_c_to_end);

            int sum_c_add_1_to_end = row_sum - prefix_sums[c + 1];
            int acc_maxc_to_c = acc_maxc_to_end - sum_c_add_1_to_end;
            curr[c] = max(curr[c], acc_maxc_to_c);
        }
    }

    // Print the maximum accmulative number.
    /* int *last_row = grid_ptr(&vals, m - 1, 0); */
    int max_score = INT_MIN;
    for (int c = 0; c < n; c += 1) {
        max_score = max(max_score, curr[c]);
    }

    printf("%d\n", max_score);

    // Free allocated memory
    grid_destroy(&vals);
    free(prefix_sums);
    free(prev);
    free(curr);

    return 0;
}
