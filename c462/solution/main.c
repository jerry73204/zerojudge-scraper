#include <assert.h>
#include <stdio.h>

int is_uppercase(char c) { return c >= 'A' && c <= 'Z'; }

int max(int a, int b) {
    if (a >= b) {
        return a;
    } else {
        return b;
    }
}

int main() {
    /* The the k parameter */
    int k;
    scanf("%d", &k);

    /* Skip a line break after k */
    char ch;
    scanf("%c", &ch);
    assert(ch == '\n');

    /* Read the first character */
    scanf("%c", &ch);
    assert(ch != '\n');

    /* Initialize the state variables for later iterations */
    int max_len = 0;  /* The max substring length so far */
    int curr_len = 0; /* The longest substring ending at the current "run" */
    int run_len = 1;  /* Current run length */
    int prev_upper = is_uppercase(
        ch); /* Whether the previous character is uppercase or not */

    /* The loop reads each character one by one. It records the length
       of the current "run", which is consists of consecutive all
       lowercase or all uppercase characters. If the letter case
       changes on the next character, it finishes a run and the run
       length is reset to 1.

       Whenever a run is finished, state variables are updated
       accordingly. It checks wheter the run length is greater or
       equal to k. If yes, increase the substring length. Otherwise,
       reset the substring length to zero or k depending on the
       current run length.
    */
    while (1) {
        /* Read the next character */
        scanf("%c", &ch);

        /* If the character is a line break, it marks the end of the
           string. */
        if (ch == '\n') {
            /* Update the max substring length if the current run
               length is not less than k. */
            if (run_len >= k) {
                max_len = max(max_len, curr_len + k);
            }
            break;
        }

        /* Get the letter case. */
        int upper = is_uppercase(ch);

        if (upper == prev_upper) {
            /* If the letter case does not change, increase the run
               length. */
            run_len += 1;
        } else {
            /* If the letter case changes, update the substring
               lengths, it marks the end of a run. Update substring
               lengths accordingly. */

            if (run_len > k) {
                /* If run length > k, set current substring length to
                   k. Also, update the max substring length so far. */
                max_len = max(max_len, curr_len + k);
                curr_len = k;
            } else if (run_len == k) {
                /* If run length == k, increase current substring
                   length by k. Also, update the max substring length
                   so far. */
                max_len = max(max_len, curr_len + k);
                curr_len += k;
            } else {
                /* If run length < k, set to substring length to
                   zero. */
                curr_len = 0;
            }

            /* Reset the run length to 1. */
            run_len = 1;
        }

        prev_upper = upper;
    }

    /* Print the maximum substring length. */
    printf("%d", max_len);
    return 0;
}
