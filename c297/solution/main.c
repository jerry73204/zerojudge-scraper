#include <stdio.h>
#include <string.h>

typedef enum play { _1B, _2B, _3B, HR, OUT } Playy;

int count_bits(int n) {
    int count = 0;
    while (n) {
        n &= n - 1;
        count += 1;
    }
    return count;
}

int main() {

    int len[9];        /* The number of games played on each player */
    Playy plays[9][5]; /* The play per player per game */
    int until_outs;    /* Print the score when this number of outs is reached */

    /* Read the table */
    for (int i = 0; i < 9; i += 1) {
        scanf("%d", &len[i]);

        for (int j = 0; j < len[i]; j += 1) {
            char code[3];
            scanf(" %2s", code);

            Playy play;
            if (!strncmp(code, "1B", 2)) {
                play = _1B;
            } else if (!strncmp(code, "2B", 2)) {
                play = _2B;
            } else if (!strncmp(code, "3B", 2)) {
                play = _3B;
            } else if (!strncmp(code, "HR", 2)) {
                play = HR;
            } else {
                play = OUT;
            }

            plays[i][j] = play;
        }
    }
    scanf("%d", &until_outs);

    /* Create state varaibles for later iterations. */
    int max_cols = len[0];
    int bases = 0;      /* The bitset marking which bases are
                           occupied or not. The 1st bit
                           corresponds to the 1st base,
                           etc. */
    int score = 0;      /* Total scores */
    int outs = 0;       /* Number of outs in the ongoing game. */
    int total_outs = 0; /* Total number of outs so far. */

    /* Enumerate each play in the table

       The outer loop iterates through the table columns. The inner
       loop iterates through plays per player in the column.
     */
    for (int col = 0; col < max_cols; col += 1) {
        for (int i = 0; i < 9; i += 1) {
            if (col == len[i]) {
                break;
            }

            int play = plays[i][col];

            switch (play) {
            case _1B: {
                bases = (bases | 1) << 1;
                score += count_bits((bases & 0x70) >> 4);
                bases &= 0x0F;
                break;
            }
            case _2B: {
                bases = (bases | 1) << 2;
                score += count_bits((bases & 0x70) >> 4);
                bases &= 0x0F;
                break;
            }
            case _3B: {
                bases = (bases | 1) << 3;
                score += count_bits((bases & 0x70) >> 4);
                bases &= 0x0F;
                break;
            }
            case HR: {
                score += count_bits(bases) + 1;
                bases = 0;
                break;
            }
            case OUT: {
                outs += 1;
                total_outs += 1;

                /* If the total number of outs reaches the limit,
                   print the score and exit. */
                if (total_outs == until_outs) {
                    printf("%d\n", score);
                    return 0;
                }

                /* If the number of outs reach to 3, reset the bases
                   bitset. */
                if (outs == 3) {
                    bases = 0;
                    outs = 0;
                }

                break;
            }
            }
        }
    }

    /* This place should not be reached. */
    return 1;
}
