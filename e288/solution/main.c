#include <assert.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const int MAX_LETTERS = 39;


int ch_to_idx(char ch) {
    if (ch >= 'A' && ch <= 'Z') {
        return (int)(ch - 'A');
    } else {
        return (int)(ch - 'a' + ('Z' - 'A' + 1));
    }
}

int compare(const void *pa, const void *pb) {
    uint64_t a = *(uint64_t *)pa;
    uint64_t b = *(uint64_t *)pb;

    if (a < b) {
        return -1;
    } else if (a > b) {
        return 1;
    } else {
        return 0;
    }
}

uint64_t lsb(uint64_t val) {
  return (val ^ (val - 1)) & val;
}

int main() {
    int m, n;
    scanf("%d %d", &m, &n);

    /* Prepare array */
    uint64_t *bitfields = (uint64_t *)malloc(sizeof(uint64_t) * n);
    assert(bitfields != NULL);

    /* Read strings */
    char buf[1001];
    uint64_t dictionary = 0;

    for (uint64_t *ptr = bitfields; ptr != bitfields + n; ptr += 1) {
        scanf("%s", buf);

        /* Each English letter correspondgs to one nth bit in the
           pattern. Here uses a bit field to record the presence of
           letters in the string. */

        uint64_t bits = 0;
        for (char *ch = buf; *ch != '\0'; ch += 1) {
            int nth = ch_to_idx(*ch);
            uint64_t bit = ((uint64_t)1) << nth;
	    dictionary |= bit;
            bits |= bit;
        }
        *ptr = bits;
    }

    uint64_t lsb_bit = lsb(dictionary);
    

    /* Provided that each bitfield represents the member set of each
       group, classify them into two kinds by checking if a specific
       member is in each group. It is done by checking LSB bit of each
       bit field.

       Here converts the bitfields. If the LSB is 1, the bitfield is
       inverted. We reuse the LSB to mark the bitfield was inverted or
       not.
    */

    for (uint64_t *ptr = bitfields; ptr != bitfields + n; ptr += 1) {
        uint64_t bits = *ptr;

        if (bits & lsb_bit) {
            *ptr = (~bits & dictionary) | lsb_bit;
        }
    }

    /* Sort the bitfields to group mutually complement groups.

       Say two groups G1 and G2 are complement to each other and
       respectively correspond to bit fields B1 and B2. Two bitfields
       B1 and B2 are complement to each other as well. One of the
       bitfields, say B2, will be inverted and marked to become B2*.
       The B1 and B2* are the same except the lowest marker bit.

       The sorting will make B1 and B2* to be adjacent to each other
       because they're almost equal except the lowest marker bit. It
       then scans adjacent converted bitfields in the sorted array to
       find such occurrences.
     */
    qsort(bitfields, n, sizeof(uint64_t), compare);

    /* printf("dict=%08" PRIx64 " lsb=%08" PRIx64 "\n", dictionary, lsb_bit); */
    /* for (uint64_t *p1 = bitfields; p1 != bitfields + n; p1 += 1) { */
    /*     printf(" %08" PRIx64 "", *p1); */
    /* } */
    /* printf("\n"); */

    /* It will scan the bitfields array. The first element will be
       processed before the scanning loop.

       The b0_count and b1_count are respective counters for
       consecutive elements marked with 0s and 1s.
    */
    int b0_count;
    int b1_count;

    if (bitfields[0] & lsb_bit) {
        b0_count = 0;
        b1_count = 1;
    } else {
        b0_count = 1;
        b1_count = 0;
    }

    int pair_count = 0;
    uint64_t prev = bitfields[0];
    bool prev_mark = (prev & lsb_bit) != 0;

    for (uint64_t *ptr = bitfields + 1; ptr != bitfields + n; ptr += 1) {
        uint64_t curr = *ptr;

        bool curr_mark = (curr & lsb_bit) != 0;

        if (prev != curr) {
            if (prev_mark) {
                if (curr_mark) {
                    b0_count = 0;
                    b1_count = 1;
                } else {
                    /* Here the former bitfield not marked while the
                       latter is not marked, which is the end of
                       complement groups. Compute the number of pairs
                       in the group. */
                    pair_count += b0_count * b1_count;
                    b0_count = 1;
                    b1_count = 0;
                }
            } else {
                if (curr_mark) {
                    /* Here the former bitfield is not marked while
                       the latter is marked. Check if the two
                       bitfields are the same except the lowest bit.
                       If yes, it finds the occurence of complement
                       groups. Otherwise, set b0_count to zero. */
                    if ((prev ^ curr) != lsb_bit) {
                        b0_count = 0;
                    }

                    b1_count = 1;
                } else {
                    b0_count = 1;
                    b1_count = 0;
                }
            }

        } else {
            if (curr_mark) {
                b1_count += 1;
            } else {
                b0_count += 1;
            }
        }

        prev = curr;
        prev_mark = curr_mark;
    }

    /* Count the pairs at the end. */
    pair_count += b0_count * b1_count;

    printf("%d\n", pair_count);
    free(bitfields);

    return 0;
}
