#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

const int MAX_SIZE = 1019;

bool is_odd(char c) { return ((c - '0') & 1) != 0; }

int compute_increment(char *num, char *inc, int even_ix) {
    int first = num[even_ix + 1];

    bool nonzero = false;	/* whethe the first nonzero digit found */
    char *inc_end = inc;	/* the place to write increment digit */
    int carry = 1;		/* the remaining value from the previous digit */

    for (int ix = even_ix + 1; num[ix] != '\n'; ix += 1) {
        carry = carry * 10 + '1' - num[ix];

	if (carry >= 10) {
	  nonzero = true;
	}

	if (nonzero) {
	  *inc_end = carry / 10 + '0';
	  inc_end += 1;
	}

        carry %= 10;
    }

    *inc_end = carry + '0';
    inc_end += 1;

    *inc_end = '\0';

    return inc_end - inc;
}

int compute_decrement(char *num, char *dec, int even_ix) {
    bool nonzero = false;	/* whethe the first nonzero digit found */
    char *dec_end = dec;	/* the place to write decrement digit */
    int carry = 1;		/* the remaining value from the previous digit */

    for (int ix = even_ix; num[ix] != '\n'; ix += 1) {
        carry = carry * 10 + num[ix] - '9';
	/* printf("# %d\n", carry); */

	if (carry >= 10) {
	  nonzero = true;
	}

	if (nonzero) {
	  *dec_end = carry / 10 + '0';
	  dec_end += 1;
	}

        carry %= 10;
    }

    *dec_end = carry + '0';
    dec_end += 1;

    *dec_end = '\0';

    return dec_end - dec;
}

int main() {
    char num[MAX_SIZE];
    char inc[MAX_SIZE];
    char dec[MAX_SIZE];

    while (true) {
        /* Read the next line. Exit the loop if it reaches EOF. */
        char *ptr = fgets(num, MAX_SIZE, stdin);
        if (ptr == NULL) {
            break;
        }

        /* int len = strnlen(num, MAX_SIZE); */

        /* Find the most significant even digit */
        int even_ix = INT_MIN;
        for (int i = 0; num[i] != '\n'; i += 1) {
            if (!is_odd(num[i])) {
                even_ix = i;
                break;
            }
        }

        /* If all digits are odd, print the answer immediately. */
        if (even_ix == INT_MIN) {
            printf("0\n");
            continue;
        }

	/* Compute the increment number */
        int inc_len = compute_increment(num, inc, even_ix);
        char *answer;

	/* Check if the decrement can possibly be smaller than the
	   increment. It's possible only when the number starts with
	   10xxx. */
        if (even_ix == 1 && num[0] == '1') {
            int dec_len = compute_decrement(num, dec, even_ix);

            if (inc_len > dec_len) {
                answer = dec;
            } else if (inc_len < dec_len) {
                answer = inc;
            } else {
                for (int ix = 0; ix < inc_len; ix += 1) {
                    int inc_digit = inc[ix];
                    int dec_digit = dec[ix];

                    if (inc_digit > dec_digit) {
                        answer = dec;
                        break;
                    } else if (inc_digit < dec_digit) {
                        answer = inc;
                        break;
                    }
                }

                answer = inc;
            }
        } else {
            answer = inc;
        }

        printf("%s\n", answer);
    }

    return 0;
}
