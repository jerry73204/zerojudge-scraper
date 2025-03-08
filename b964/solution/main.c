/*
  Submisssion ID: 15613512
  AC (2ms, 84KB)
 */

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

int min(int a, int b);
int max(int a, int b);
int compare(const void *a, const void *b);

int main() {
  /* Read the number of grades to read. */
  int n;
  scanf("%d", &n);

  /* Check the special case when n == 0. */
  if (n == 0) {
    printf("worst case\n");
    printf("best case\n");
    return 0;
  }

  /* Read the grade list. It finds the max failing grade and min
     passing grade during iterations. */
  int max_fail = INT_MIN;
  int min_pass = INT_MAX;

  int *grades = (int *)malloc(n * sizeof(int));
  assert(grades != NULL);

  for (int i = 0; i < n; i += 1) {
    int grade;
    scanf("%d", &grade);
    grades[i] = grade;

    if (grade >= 60) {
      min_pass = min(min_pass, grade);
    } else {
      max_fail = max(max_fail, grade);
    }
  }

  /* Sort the grades. */
  qsort(grades, n, sizeof(int), compare);

  /* Print the sorted grades */
  printf("%d", grades[0]);
  for (int i = 1; i < n; i += 1) {
    printf(" %d", grades[i]);
  }
  printf("\n");

  /* Print the maximum failing grade */
  if (max_fail == INT_MIN) {
    printf("best case\n");
  } else {
    printf("%d\n", max_fail);
  }

  /* Print the minimum passing grade */
  if (min_pass == INT_MAX) {
    printf("worst case\n");
  } else {
    printf("%d\n", min_pass);
  }

  /* Finalize */
  free(grades);

  return 0;
}

/* This callback function is used by qsort(). */
int compare(const void *a, const void *b) {
  int na = *(const int *)a;
  int nb = *(const int *)b;

  if (na < nb) {
    return -1;
  } else if (na > nb) {
    return 1;
  } else {
    return 0;
  }
}

int min(int a, int b) {
  if (a <= b) {
    return a;
  } else {
    return b;
  }
}

int max(int a, int b) {
  if (a >= b) {
    return a;
  } else {
    return b;
  }
}
