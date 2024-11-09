#include <assert.h>
#include <stdio.h>

int main() {
  /* Read the side lengths of a triangle */
  int ia, ib, ic;
  scanf("%d %d %d", &ia, &ib, &ic);

  /* Sort the side lengths */
  int sa, sb, sc;
  if (ia > ib) {
    if (ia < ic) {
      sa = ib;
      sb = ia;
      sc = ic;
    } else if (ib > ic) {
      sa = ic;
      sb = ib;
      sc = ia;
    } else {
      sa = ib;
      sb = ic;
      sc = ia;
    }
  } else {
    if (ib < ic) {
      sa = ia;
      sb = ib;
      sc = ic;
    } else if (ia > ic) {
      sa = ic;
      sb = ia;
      sc = ib;
    } else {
      sa = ia;
      sb = ic;
      sc = ib;
    }
  }

  assert(sa <= sb && sb <= sc);

  /* Determine the type of the triangle. */
  if (sa + sb <= sc) {
    printf("No\n");
  } else {

    int ab_sqsum = sa * sa + sb * sb;
    int c_sq = sc * sc;

    if (ab_sqsum < c_sq) {
      printf("Obtuse\n");
    } else if (ab_sqsum > c_sq) {
      printf("Acute\n");
    } else {
      printf("Right\n");
    }
  }

  return 0;
}
