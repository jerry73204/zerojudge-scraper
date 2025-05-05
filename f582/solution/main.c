#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

char *get_line(char *buf, int len, int row) {
  return &buf[row * (len + 1)];
}

int main() {
    int n, m;
    scanf("%d %d", &n, &m);

    int *parents = (int *) malloc(sizeof(int) * n);
    assert(parents != NULL);

    char *sequences = (char*) malloc(sizeof(char) * n * (m + 1));
    assert(sequences != NULL);

    for (int ix = 0; ix < n; ix += 1) {
      int dst, src;
      scanf("%d %d ", &dst, &src);
      dst -= 1;
      src -= 1;
      parents[dst] = src;
      fgets(get_line(sequences, m, dst), m, stdin);
    }

    free(parents);
    free(sequences);
    
  return 0;
}
