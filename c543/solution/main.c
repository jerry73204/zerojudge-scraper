#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define MAX_LEN 100002

int64_t MOD = 1000000007;
char NUMBER[MAX_LEN];  /* The buffer that stores the input number. */
int64_t FACT[MAX_LEN]; /* The array of modulo factorials */

/* Compute the greatest common divisor between na and nb and compute
   the coefficients ca and cb such that ca * na + cb * nb == gcd. */
int64_t gcd(int64_t *ca, int64_t na, int64_t *cb, int64_t nb) {
    if (na == 0) {
        *ca = 0;
        *cb = 1;
        return nb;
    } else if (nb == 0) {
        *ca = 1;
        *cb = 0;
        return na;
    } else if (na < nb) {
        return gcd(cb, nb, ca, na);
    } else {
        int64_t quot = na / nb;
        int64_t g = gcd(cb, nb, ca, na % nb);
        *cb = *cb - quot * (*ca);
        return g;
    }
}

/* Compute the modulo inverse of n. */
int inverse(int64_t n) {
    int64_t cn, cp;
    int64_t g = gcd(&cn, n, &cp, MOD);

    cn = cn % MOD;
    if (cn < 0) {
        cn += MOD;
    }

    return cn;
}

/* Compute the combination with params n, k */
int64_t comb(int64_t n, int64_t k) {
    int64_t fn = FACT[n];
    int64_t fa = inverse(FACT[k]);
    int64_t fb = inverse(FACT[n - k]);
    return (((fn * fa) % MOD) * fb) % MOD;
}

/* Compute the combination with repetitions with params n, k */
int64_t rcomb(int64_t n, int64_t k) { return comb(n + k - 1, k); }

int main() {
    /* Pre-compute the module factorial array */
    FACT[0] = 1;
    for (int i = 1; i < MAX_LEN; i += 1) {
        FACT[i] = (FACT[i - 1] * i) % MOD;
    }

    /* The loop reads the numbers line by line. */
    while (scanf("%100001s", NUMBER) != EOF) {

        /* Initialize the state variables */
        int n = strnlen(NUMBER, MAX_LEN);
        int64_t lower = 0; /* The minimum number in later digits */
        int64_t count = 0; /* Total count so far */

        /* Iterate from the highest digit to the lowest digit. In the
           nth loop, the leading n digits are fixed and count the
           combinations of remaining digits. */
        for (int nth = 0; nth < n; nth += 1) {
            int64_t upper = NUMBER[nth] - '0';

            if (lower > upper) {
                break;
            }

            for (int64_t d = lower; d < upper; d += 1) {
                count = (count + rcomb(n - nth, 9 - d)) % MOD;
            }

            lower = upper;
        }

        printf("%d\n", (int)count);
    }
    return 0;
}
