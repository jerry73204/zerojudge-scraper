/*
  Submission ID: 15613523
  AC (6ms, 92KB)
 */

#include <stdio.h>

int abs(int n) {
    if (n >= 0) {
        return n;
    } else {
        return -n;
    }
}

int main() {
    /* a_sum and b_sum are the sums of digits in odd end even
       places. */
    int a_sum = 0;
    int b_sum = 0;

    /* The number of the digits of the number. */
    int length = 0;

    /* Read digits one-by-one until a line break is encountered. */
    while (1) {
        char ch;
        scanf("%c", &ch);

        if (ch == '\n') {
            break;
        } else {
            length += 1;

            if (length & 1) {
                a_sum += (int)(ch - '0');
            } else {
                b_sum += (int)(ch - '0');
            }
        }
    }

    /* Print the absolute difference of odd and even digits. */
    int ans = abs(a_sum - b_sum);
    printf("%d\n", ans);

    return 0;
}
