#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void swap(int *a, int *b);

int main() {
  /* Read the number of rows, columns and operations. */
  int r, c, m;
  scanf("%d %d %d", &r, &c, &m);

  /* Early exit if the number of elements is zero. */
  int num_el = r * c;
  if (num_el == 0) {
    return 0;
  }

  /* Read matrix data */
  int *data = (int *)malloc(num_el * sizeof(int));
  assert(data != NULL);

  for (int i = 0; i < num_el; i += 1) {
    scanf("%d", &data[i]);
  }

  /* `num_flips` records the number of flip operations. */
  int num_flips = 0;

  /* `corner` records the corner index of the matrix placed on the
     top left place after a series of operations. The corner index
     counts from 0 to 3, starting from the top left element of the
     matrix before any operations.
   */
  int corner = 0;

  /* Scan operations codes. */
  for (int i = 0; i < m; i += 1) {
    int op;
    scanf("%d", &op);

    if (op) {
      /* op == 1: vertical flip */
      num_flips += 1;
      corner = (6 - corner) % 4;
    } else {
      /* op == 0: rotation */
      corner = (corner + 1) % 4;
    }
  }

  /* Change `corner` and `transposed` to the number of `rotations`
     and a `tranpose` or not. For example, the matrix flipped once
     equals to the matrix rotated once and then transposed. */
  int transposed = num_flips & 1;
  int rotations;
  if (transposed) {
    rotations = 3 - corner;
  } else {
    rotations = corner;
  }

  /* Find the starting element index, the row step and the column
     step when tranversing the rotated matrix before transpose.

     `start` is the first printed element index in `data`.

     `row_step` is the number of index advance from the current
     element to the element in the next row in `data`.

     `column_step` is the number of index advance from the current
     element to the element in the next column in `data`.
  */
  int start;
  int row_step;
  int col_step;

  switch (rotations) {
  case 0:
    start = 0;
    row_step = c;
    col_step = 1;
    break;
  case 1:
    start = (r - 1) * c;
    row_step = 1;
    col_step = -c;
    break;
  case 2:
    start = r * c - 1;
    row_step = -c;
    col_step = -1;
    break;
  case 3:
    start = c - 1;
    row_step = -1;
    col_step = c;
    break;
  }

  /* Find the number of maximum number of row and column steps when
     tranversing the rotated matrix before transpose. */
  int row_max;
  int col_max;

  if (rotations & 1) {
    row_max = c;
    col_max = r;
  } else {
    row_max = r;
    col_max = c;
  }

  /* If the matrix is transposed, swap the row and column steps. */
  if (transposed) {
    swap(&row_step, &col_step);
    swap(&row_max, &col_max);
  }

  /* Print the number rows and columns of the printed matrix. */
  printf("%d %d\n", row_max, col_max);

  /* Travere and print the elements in `data`. The outer loop
     traverse through the rows, while the inner loop travers through
     the elements in a row. */
  int row_index = start;
  for (int ri = 0; ri < row_max; ri += 1) {
    int col_index = row_index;

    /* Print the 1st element in the current row. */
    printf("%d", data[col_index]);
    col_index += col_step;

    /* Print the 2nd and later elements in the current row. */
    for (int ci = 1; ci < col_max; ci += 1) {
      printf(" %d", data[col_index]);
      col_index += col_step;
    }

    printf("\n");
    row_index += row_step;
  }

  return 0;
}

void swap(int *a, int *b) {
  *a ^= *b;
  *b ^= *a;
  *a ^= *b;
}
