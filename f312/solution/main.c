#include <inttypes.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>


int64_t eval(int64_t a, int64_t b, int64_t c, int64_t x) {
  return a * x * x + b * x + c;
}

int64_t max(int64_t a, int64_t b) {
  if (a >= b) {
    return a;
  } else {
    return b;
  }
}

int main() {
  int64_t a1, b1, c1, a2, b2, c2, n;
  scanf("%" SCNd64 " %" SCNd64 " %" SCNd64 "", &a1, &b1, &c1);
  scanf("%" SCNd64 " %" SCNd64 " %" SCNd64 "", &a2, &b2, &c2);
  scanf("%" SCNd64 "", &n);

  /* Compute the coefficients of the formula
     f(x) = (a1 x^2 + b1 x + c1) + a2 (n-x)^2 + b2 (n-x) + c2
  */
  int64_t k2 = a1 + a2;
  int64_t k1 = b1 - b2 - 2 * a2 * n;
  int64_t k0 = a2 * n * n + b2 * n + c1 + c2;

  int64_t answer;

  if (k2 >= 0) {
    /* When k2 is non-negative, the optimal points is either 0 or
       n. */

    int64_t f0 = eval(k2, k1, k0, 0);
    int64_t fn = eval(k2, k1, k0, n);
    answer = max(f0, fn);
  } else {
    /* The optimial point will be around x = k1 / (-2 * k2). The k2 is
       negative in this block. The sign of optimial x is determined by
       k1.

       In the case that k1 is non-positive, x becomes non-positive,
       too. The maximum value will be on x = 0.

       Otherwise, x is positive. It checks which of floor(x) and
       ceil(x) gives the optimal value.
     */

    if (k1 <= 0) {
      answer = eval(k2, k1, k0, 0);
    } else {
      double optimal_x = (double) k1 / (-2.0 * (double) k2);

      if (optimal_x >= n) {
	answer = eval(k2, k1, k0, n);
      } else {
	int64_t xa = floor(optimal_x);
	int64_t xb = ceil(optimal_x);

	int64_t fa = eval(k2, k1, k0, xa);
	int64_t fb = eval(k2, k1, k0, xb);

	answer = max(fa, fb);
      }
    }
  }

  /* Print the answer */
  printf("%" SCNd64, answer);

  return 0;
}
