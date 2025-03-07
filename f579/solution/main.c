/*
  Submission ID: 15608614
  AC (2ms, 104KB)
 */

#include <stdio.h>

int main() {
    int a, b, n;
    scanf("%d %d %d", &a, &b, &n);

    int customer_cnt = 0;

    for (int i = 0; i < n; i += 1) {
        int t; /* transaction */
        int a_cnt = 0;
        int b_cnt = 0;

        while (1) {
            scanf("%d", &t);
            if (t == 0) {
                break;
            } else if (t == a) {
                a_cnt += 1;
            } else if (t == b) {
                b_cnt += 1;
            } else if (t == -a) {
                a_cnt -= 1;
            } else if (t == -b) {
                b_cnt -= 1;
            }
        }

        if (a_cnt > 0 && b_cnt > 0) {
            customer_cnt += 1;
        }
    }

    printf("%d\n", customer_cnt);
    return 0;
}
