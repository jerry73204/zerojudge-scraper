#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

int main() {
  int m, n;
  scanf("%d %d", &n, &m);

  int *weights = (int*) calloc(2 * n, sizeof(int));
  assert(weights != NULL);
  for (int i = n; i < 2 * n; i += 1) {
    scanf("%d", &weights[i]);
  }

  int *cargos = (int*) malloc(sizeof(int) * m);
  assert(cargos != NULL);
  for (int *ptr = cargos; ptr != cargos + m; ptr +=1) {
    scanf("%d", ptr);
  }

  int *lefts = (int*) malloc(sizeof(int) * n);
  assert(lefts != NULL);

  int *rights = (int*) malloc(sizeof(int) * n);
  assert(rights != NULL);

  int *parents = (int*) malloc(sizeof(int) * (2 * n));
  assert(parents != NULL);
  parents[1] = 0;

  for (int i = 1; i < n; i += 1) {
    int a, b, c;
    scanf("%d %d %d", &a, &b, &c);

    lefts[a] = b;
    rights[a] = c;
    parents[b] = a;
    parents[c] = a;
  }

  for (int i = n; i < 2 * n; i += 1) {
    int weight = weights[i];
    int curr = i;

    while (parents[curr] != 0) {
      curr = parents[curr];
      weights[curr] += weight;
    }
  }

  bool is_first = true;

  for (int *ptr = cargos; ptr != cargos + m; ptr +=1) {
    int cargo_weight = *ptr;

    int curr = 1;
    while (curr < n) {
      int left = lefts[curr];
      int right = rights[curr];

      int *curr_weight = &weights[curr];
      int *left_weight = &weights[left];
      int *right_weight = &weights[right];

      *curr_weight += cargo_weight;

      if (*left_weight <= *right_weight) {
	curr = left;
      } else {
	curr = right;
      }
    }

    weights[curr] += cargo_weight;

    if (is_first) {
      is_first = false;
    } else {
      printf(" ");
    }
    printf("%d", curr);
  }

  printf("\n");

  return 0;
}
