#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

static int ROW_STEPS[4] = {0, -1, 0, 1};
static int COL_STEPS[4] = {-1, 0, 1, 0};

int main() {
  /* Read the matrix size */
  int n;
  scanf("%d", &n);

  /* Read the initial direction */
  int direction;
  scanf("%d", &direction);

  /* Read the matrix */
  int *matrix = (int *)malloc(n * n * sizeof(int));
  assert(matrix != NULL);

  for (int *ptr = matrix; ptr != matrix + n * n; ptr += 1) {
    scanf("%d", ptr);
  }

  /* Initialize the position */
  int row = n / 2;
  int col = n / 2;

  /* Traverse the matrix elements in the spiral order.

     The traversal consists of two loops. The outer loop counts the
     "steps" from 1 to n-1. The inner loop repeats 2 times. In each
     inner loop, the cursor moves "steps" times, in which each move
     prints the element.
   */
  for (int steps = 1; steps < n; steps += 1) {
    for (int i = 0; i < 2; i += 1) {
      int rs = ROW_STEPS[direction];
      int cs = COL_STEPS[direction];

      for (int j = 0; j < steps; j += 1) {
        printf("%d", matrix[row * n + col]);
        row += rs;
        col += cs;
      }

      direction = (direction + 1) % 4;
    }
  }

  /* Print remaining elements */
  int rs = ROW_STEPS[direction];
  int cs = COL_STEPS[direction];
  for (int j = 0; j < n; j += 1) {
    printf("%d", matrix[row * n + col]);
    row += rs;
    col += cs;
  }

  /* Print the trailing line break. */
  printf("\n");

  /* Finalize */
  free(matrix);

  return 0;
}
