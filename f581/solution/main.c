/*
   Submission ID: 15610397
   AC (27ms, 1.6MB)
 */

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct binary_search_t {
    int index;
    bool exact;
} BinarySearch;

BinarySearch binary_search(int *nums, int n, int target) {
    int lix = 0;
    int rix = n;

    while (1) {
        if (lix == rix) {
            BinarySearch result;
            result.index = lix;
            result.exact = false;
            return result;
        }

        int mix = lix + (rix - lix) / 2;
        int pivot = nums[mix];

        if (target < pivot) {
            rix = mix;
        } else if (target > pivot) {
            lix = mix + 1;
        } else {
            BinarySearch result;
            result.index = mix;
            result.exact = true;
            return result;
        }
    }
}

int main() {
    int n, m;
    scanf("%d %d", &n, &m);

    int *ps = (int*)malloc(sizeof(int) * n);
    assert(ps != NULL);

    for (int *ptr = ps; ptr != ps + n; ptr += 1) {
        scanf("%d", ptr);
    }

    int *prefix_sums = (int*)malloc(sizeof(int) * (n + 1));
    assert(prefix_sums != NULL);


    prefix_sums[0] = 0;
    for (int i = 0; i < n; i += 1) {
        prefix_sums[i + 1] = prefix_sums[i] + ps[i];
    }

    int p_sum = prefix_sums[n];
    int ix = 0;

    for (int i = 0; i < m; i += 1) {
        int q;
        scanf("%d", &q);
        int target = (prefix_sums[ix] + q) % p_sum;
        BinarySearch result = binary_search(prefix_sums, n + 1, target);
	ix = result.index % n;
    }

    printf("%d\n", ix);

    free(ps);
    free(prefix_sums);
    return 0;
}
