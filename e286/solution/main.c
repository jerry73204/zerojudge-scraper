#include <stdio.h>
int main() {
    int wins = 0;
    int home = 0;
    int away = 0;

    /* Read the 1st line */
    for (int i = 0; i < 4; i += 1) {
        int score;
        scanf("%d", &score);
        home += score;
    }

    /* Read the 2nd line */
    for (int i = 0; i < 4; i += 1) {
        int score;
        scanf("%d", &score);
        away += score;
    }

    /* Check win or lose */
    if (home > away) {
        wins += 1;
    }

    /* Print the scores in the first game */
    printf("%d:%d\n", home, away);

    /* Reset the scores to zero */
    home = 0;
    away = 0;

    /* Read the 3rd line */
    for (int i = 0; i < 4; i += 1) {
        int score;
        scanf("%d", &score);
        home += score;
    }

    /* Read the 4th line */
    for (int i = 0; i < 4; i += 1) {
        int score;
        scanf("%d", &score);
        away += score;
    }

    /* Check win or lose */
    if (home > away) {
        wins += 1;
    }

    /* Print the scores in the second game */
    printf("%d:%d\n", home, away);

    /* Print the verdict */
    if (wins > 0) {
        printf("Win\n");
    } else if (wins < 0) {
        printf("Lose\n");
    } else {
        printf("Tie\n");
    }

    return 0;
}
