/*
  Submission ID: 15609451
  AC (2ms, 88KB)
 */

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int FRONT_MAP[6] = {2, 1, 5, 0, 4, 3};
int RIGHT_MAP[6] = {4, 0, 2, 3, 5, 1};

typedef struct op {
    int a;
    int b;
} Op;

void swap(int *a, int *b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int main() {
    int n, m;

    scanf("%d %d", &n, &m);

    int *dices = (int *)malloc(sizeof(int) * n);
    assert(dices != NULL);
    memset(dices, 0, sizeof(int) * n);

    Op *ops = (Op *)malloc(sizeof(Op) * m);
    assert(ops != NULL);

    for (Op *ptr = ops; ptr != ops + m; ptr += 1) {
        scanf("%d %d", &ptr->a, &ptr->b);
    }

    {
        Op *ptr = ops + m - 1;

        while (1) {
            if (ptr->b == -1) {
                int ix = ptr->a - 1;
                dices[ix] = FRONT_MAP[dices[ix]];
            } else if (ptr->b == -2) {
                int ix = ptr->a - 1;
                dices[ix] = RIGHT_MAP[dices[ix]];
            } else {
                swap(&dices[ptr->a - 1], &dices[ptr->b - 1]);
            }

            if (ptr == ops) {
                break;
            }
            ptr -= 1;
        }
    }

    for (Op *ptr = ops; ptr != ops + m; ptr += 1) {
        if (ptr->b > 0) {
            swap(&dices[ptr->a - 1], &dices[ptr->b - 1]);
        }
    }

    printf("%d", dices[0] + 1);
    for (int *ptr = dices + 1; ptr != dices + n; ptr += 1) {
        printf(" %d", *ptr + 1);
    }
    printf("\n");

    free(ops);
    free(dices);

    return 0;
}
