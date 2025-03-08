/*
  Submission ID: 15613602
  AC (54ms, 3.1MB)
 */

#include <assert.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct objcet_t {
    int64_t w;
    int64_t f;
} Object;

int compare(const void *p1, const void *p2) {
    const Object *obj1 = (const Object *)p1;
    const Object *obj2 = (const Object *)p2;
    int a = obj1->w * obj2->f;
    int b = obj2->w * obj1->f;
    return a - b;
}

int main() {
    /* read the number of objects */
    int n;
    scanf("%d", &n);

    /* prepare arrays */
    Object *obj = (Object *)malloc(sizeof(Object) * n);
    assert(obj != NULL);

    /* read object data */
    for (Object *ptr = obj; ptr != obj + n; ptr += 1) {
        scanf("%" SCNd64 "", &ptr->w);
    }
    for (Object *ptr = obj; ptr != obj + n; ptr += 1) {
        scanf("%" SCNd64 "", &ptr->f);
    }

    /* sort objects */
    qsort(obj, n, sizeof(Object), compare);

    /* compute the energy */
    int64_t acc_w = 0;
    int64_t energy = 0;

    for (Object *ptr = obj; ptr != obj + n; ptr += 1) {
        energy += acc_w * ptr->f;
        acc_w += ptr->w;
    }

    printf("%" PRId64 "\n", energy);
    free(obj);
    return 0;
}
