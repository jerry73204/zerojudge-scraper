#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int min(int a, int b) {
  if (a <= b) {
    return a;
  } else {
    return b;
  }
}

int main() {
  int n, target, lsteps, rsteps;
  scanf("%d %d %d %d", &n, &target, &lsteps, &rsteps);

  int *gates = (int*) malloc(sizeof(int) * n);
  assert(gates != NULL);

  for (int *ptr = gates; ptr != gates + n; ptr += 1) {
    scanf("%d", ptr);
  }

  int *lefts = (int*) malloc(sizeof(int) * n);
  assert(lefts != NULL);

  int *rights = (int*) malloc(sizeof(int) * n);
  assert(rights != NULL);

  for (int ix = lsteps; ix < n; ix += 1) {
    int lix = ix - lsteps;
    lefts[ix] = gates[lix];
  }
  lefts[target] = target;

  for (int ix = 0; ix < n - rsteps; ix += 1) {
    int rix = ix + rsteps;
    rights[ix] = gates[rix];
  }
  rights[target] = target;

  int *distances = (int*) malloc(sizeof(int) * n);
  assert(distances != NULL);

  for (int *ptr = distances; ptr != distances + n; ptr += 1) {
    *ptr = INT_MAX;
  }
  distances[target] = 0;

  for (int r = 1; r < n; r += 1) {
    for (int curr = lsteps; curr < n; curr += 1) {
      int next = lefts[curr];
      distances[curr] = min(distances[curr], distances[next] + 1);
    }

    for (int curr = 0; curr < n - rsteps; curr += 1) {
      int next = rights[curr];
      distances[curr] = min(distances[curr], distances[next] + 1);
    }
  }

  printf("%d\n", distances[0]);

  free(gates);
  free(lefts);
  free(rights);

  return 0;
}
