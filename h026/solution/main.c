#include <stdio.h>

// Function to determine the winner based on two throws
// Returns:  1 if brother wins
//          -1 if sister wins
//           0 if it's a tie
int determineWinner(int brother, int sister) {
    // 0: rock, 2: scissors, 5: paper
    if (brother == sister) {
        return 0; // Tie
    } else if ((brother == 0 && sister == 2) || // Rock beats scissors
               (brother == 2 && sister == 5) || // Scissors beats paper
               (brother == 5 && sister == 0)) { // Paper beats rock
        return 1; // Brother wins
    } else {
        return -1; // Sister wins
    }
}

// Function to get the winning move against a given move
int getWinningMove(int move) {
    // 0: rock -> 5: paper (to win)
    // 2: scissors -> 0: rock (to win)
    // 5: paper -> 2: scissors (to win)
    if (move == 0) return 5;
    if (move == 2) return 0;
    return 2; // if move == 5
}

int main() {
    int F, N;
    int sisterMoves[10]; // Max N is 10
    int brotherMoves[11]; // Including the initial move
    
    // Read initial move of brother
    scanf("%d", &F);
    brotherMoves[0] = F;
    
    // Read number of rounds
    scanf("%d", &N);
    
    // Read sister's moves
    for (int i = 0; i < N; i++) {
        scanf("%d", &sisterMoves[i]);
    }
    
    // Simulate the game
    int roundResult = 0;
    int resultRound = 0;
    
    for (int round = 0; round < N; round++) {
        // Determine current round result
        roundResult = determineWinner(brotherMoves[round], sisterMoves[round]);
        
        // If not a tie, record the round when the result is determined
        if (roundResult != 0) {
            resultRound = round + 1;
            break;
        }
        
        // Determine brother's next move
        if (round >= 1 && sisterMoves[round] == sisterMoves[round - 1]) {
            // If sister throws the same move twice in a row,
            // brother throws the move that would beat sister's last two identical moves
            brotherMoves[round + 1] = getWinningMove(sisterMoves[round]);
        } else {
            // Otherwise, brother copies sister's last move
            brotherMoves[round + 1] = sisterMoves[round];
        }
    }
    
    // Print brother's moves
    for (int i = 0; i <= (resultRound > 0 ? resultRound - 1 : N - 1); i++) {
        printf("%d", brotherMoves[i]);
        if (i < (resultRound > 0 ? resultRound - 1 : N - 1)) {
            printf(" ");
        }
    }
    
    // Print result
    if (resultRound == 0) {
        printf(" : Drew at round %d", N);
    } else if (roundResult == 1) {
        printf(" : Won at round %d", resultRound);
    } else if (roundResult == -1) {
        printf(" : Lost at round %d", resultRound);
    }
    
    printf("\n");
    
    return 0;
}