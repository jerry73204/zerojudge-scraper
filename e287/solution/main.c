/*
  Submission ID: 15613622
  AC (3ms, 120KB)
 */

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

int min(int a, int b) {
    if (a <= b) {
        return a;
    } else {
        return b;
    }
}

int main() {
    /* Read the map size */
    int n, m;
    scanf("%d %d", &n, &m);

    /* Initialize the map array */
    int *map = (int *)malloc(n * m * sizeof(int));
    assert(map != NULL);

    /* Read the first value of the map. Record it as the current
       minimum value so far. */
    scanf("%d", &map[0]);

    int min_val = map[0];
    int min_idx = 0;

    /* Read remaining values of the map. Find the minimum value and
       its index in the mean time. */
    for (int *ptr = map + 1; ptr != map + n * m; ptr += 1) {
        scanf("%d", ptr);

        if (*ptr < min_val) {
            min_val = *ptr;
            min_idx = ptr - map;
        }
    }

    /* Set the initial position to the place of minimum value. */
    int row = min_idx / m;
    int col = min_idx % m;
    int sum = 0;

    /* Run the traversal. In each loop, find the unvisited neighbor
       cell with the minimum value. */
    while (1) {
        /* Add the current cell value to the sum variable. */
        int *curr = &map[row * m + col];
        sum += *curr;
        *curr = INT_MAX;

        /* Find the unvisited neighbor cell with the minimum value. */
        int next_row;
        int next_col;
        int next_val = INT_MAX;

        if (row - 1 >= 0) {
            int val = map[(row - 1) * m + col];
            if (next_val > val) {
                next_val = val;
                next_row = row - 1;
                next_col = col;
            }
        }

        if (row + 1 < n) {
            int val = map[(row + 1) * m + col];
            if (next_val > val) {
                next_val = val;
                next_row = row + 1;
                next_col = col;
            }
        }

        if (col - 1 >= 0) {
            int val = map[row * m + (col - 1)];
            if (next_val > val) {
                next_val = val;
                next_row = row;
                next_col = col - 1;
            }
        }

        if (col + 1 < m) {
            int val = map[row * m + (col + 1)];
            if (next_val > val) {
                next_val = val;
                next_row = row;
                next_col = col + 1;
            }
        }

        /* If the minimum neighbor cell value is INT_MAX, that is, all
           neighbors are visited, stop the iteration. */
        if (next_val == INT_MAX) {
            break;
        } else {
            row = next_row;
            col = next_col;
        }
    }

    /* Print the result */
    printf("%d\n", sum);

    /* Finalize */
    free(map);
    return 0;
}
