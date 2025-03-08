/*
  Submission ID: 15613548
  AC (2ms, 88KB)
 */

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  int n, m;
  scanf("%d %d", &n, &m);

  int *values = (int *)malloc(n * m * sizeof(int));
  assert(values != NULL);

  for (int *ptr = values; ptr < values + n * m; ptr += 1) {
    scanf("%d", ptr);
  }

  int sum = 0;
  int *maxvals = (int *)malloc(n * sizeof(int));
  assert(maxvals != NULL);
  int *maxvals_end = maxvals;

  for (int row = 0; row < n; row += 1) {
    int maxval = values[row * m];

    for (int col = 1; col < m; col += 1) {
      int val = values[row * m + col];
      if (maxval <= val) {
        maxval = val;
      }
    }

    *maxvals_end = maxval;
    maxvals_end += 1;

    sum += maxval;
  }

  printf("%d\n", sum);

  int has_divisor = 0;
  int *ptr = maxvals;

  while (ptr != maxvals_end) {
    int val = *ptr;
    ptr += 1;

    if ((sum % val) == 0) {
      has_divisor = 1;
      printf("%d", val);
      break;
    }
  }

  while (ptr != maxvals_end) {
    int val = *ptr;
    ptr += 1;

    if ((sum % val) == 0) {
      has_divisor = 1;
      printf(" %d", val);
    }
  }

  if (has_divisor) {
    printf("\n");
  } else {
    printf("-1\n");
  }

  /* Finalize */
  free(values);
  return 0;
}
