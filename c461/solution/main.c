#include <stdio.h>

int main() {
    int a, b, c;
    scanf("%d %d %d", &a, &b, &c);

    int impossible = 1;

    if ((a && b) == c) {
        printf("AND\n");
        impossible = 0;
    }

    if ((a || b) == c) {
        printf("OR\n");
        impossible = 0;
    }

    if ((!!a ^ !!b) == c) {
        printf("XOR\n");
        impossible = 0;
    }

    if (impossible) {
        printf("IMPOSSIBLE\n");
    }

    return 0;
}
