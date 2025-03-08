/*
  Submission ID: 15613516
  AC (10ms, 396KB)
 */

#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct marker {
    int pos;
    int end;
} Marker;

int compare(const void *a, const void *b) {
    Marker *ma = (Marker *)a;
    Marker *mb = (Marker *)b;

    if (ma->pos < mb->pos) {
        return -1;
    } else if (ma->pos > mb->pos) {
        return 1;
    } else if (ma->end < mb->end) {
        return -1;
    } else if (ma->end > mb->end) {
        return 1;
    } else {
        return 0;
    }
}

int main() {
    /* Read the number of ranges. */
    int n;
    scanf("%d", &n);

    /* Initialize the marker array. Each marker is a start or an end of
       a range. */
    Marker *markers = (Marker *)malloc(2 * n * sizeof(Marker));
    assert(markers != NULL);

    /* Scan the ranges and split each of them to a start and an end
       markers. Store the markers in the array. */
    for (Marker *ptr = markers; ptr != markers + n * 2; ptr += 2) {
        int start, end;
        scanf("%d %d", &start, &end);

        ptr->pos = start;
        ptr->end = 0;

        (ptr + 1)->pos = end;
        (ptr + 1)->end = 1;
    }

    /* Sort the markers by their position */
    qsort(markers, 2 * n, sizeof(Marker), compare);

    /* Interate through the sorted markers and keep track of the number
       of overlapped ranges. */
    int count = 0;
    int start;
    int length = 0;

    for (Marker *ptr = markers; ptr != markers + n * 2; ptr += 1) {
        if (ptr->end) {
            /* In case that an ending marker is visited, decrease the
               overlap count. If the count decreases to zero, add the
               overlap length to the total length. */

            count -= 1;

            if (count == 0) {
                length += ptr->pos - start;
            }
        } else {
            /* In case that a starting marker is visited, increase the
               overlap count. */
            if (count == 0) {
                start = ptr->pos;
            }
            count += 1;
        }
    }

    /* Print the total length. */
    printf("%d\n", length);

    /* Finalize */
    free(markers);

    return 0;
}
