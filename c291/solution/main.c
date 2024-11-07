#include <stdio.h>
#include <stdlib.h>

int main() {
  /* Read the number of people. */
  int n;
  scanf("%d", &n);

  /* Read the friends of each person. */
  int *friends = (int *)malloc(n * sizeof(int));
  for (int *ptr = friends; ptr != friends + n; ptr += 1) {
    scanf("%d", ptr);
  }

  /* Initialize the conut that will be the answer. */
  int count = 0;

  /* Iterate through the firend array. */
  for (int *start = friends; start != friends + n; start += 1) {
    /* Skip if the friend number is erased. */
    if (*start == -1) {
      continue;
    }

    /* Increase the group conut. */
    count += 1;

    /* Visit the friends and firends of friends and erase them. */
    int *curr = start;
    while (*curr != -1) {
      int *next = friends + *curr;
      *curr = -1;
      curr = next;
    }
  }

  /* Print the answer. */
  printf("%d\n", count);

  /* Finalize */
  free(friends);

  return 0;
}
