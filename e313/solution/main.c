#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

const int MAX_SIZE = 1001;

int count_distinct(char *buf) {
    int count = 0; /* The number of distinct letters */

    /* The boolean array records the presence of each letter */
    bool letters[26];
    memset(letters, 0, 26 * sizeof(bool));

    for (char *ptr = buf; *ptr != '\n'; ptr += 1) {
        int nth = *ptr - 'A';

        if (!letters[nth]) {
            letters[nth] = true;
            count += 1;
        }
    }

    return count;
}

void swap(char **p1, char **p2) {
    char *tmp = *p1;
    *p1 = *p2;
    *p2 = tmp;
}

int main() {
    int n;
    char buf1[MAX_SIZE];
    char buf2[MAX_SIZE];

    /* Read the number of strings */
    scanf("%d", &n);
    getc(stdin); /* Drop the trailing '\n' */

    /* Bail if there is no string to read */
    if (n == 0) {
        return 0;
    }

    /* Read the first line of string */
    fgets(buf1, MAX_SIZE, stdin);

    /* Set the answer to the first line of string for now. */
    char *answer = buf1;
    char *candidate = buf2;
    int min_distinct = count_distinct(answer);

    for (int i = 1; i < n; i += 1) {
        /* Get the current candidate string and count distinct
           letters. */
        fgets(candidate, MAX_SIZE, stdin);
        int n_distinct = count_distinct(candidate);

        if (n_distinct < min_distinct) {
            /* Case 1

               If the candidate has less distinct letters, set the current
               candidate as the answer and reuse the old answer buffer to
               read the next candidate. */
            min_distinct = n_distinct;
            swap(&answer, &candidate);
        } else if (n_distinct == min_distinct) {
            /* Case 2

               If the candidate has equal number of distinct letters with
               the current answer, check the dictionary order.

               Note the direct strncmp() is OK because even one string is
               shorder, the ending '\n' has smaller ASCII code than
               English letters.
             */
            if (strncmp(candidate, answer, MAX_SIZE) < 0) {
                swap(&answer, &candidate);
            }
        }
    }

    /* Print the answer The answer already has a trailing '\n' so the
       format string does not add the extra '\n'.
     */
    printf("%s", answer);

    return 0;
}
