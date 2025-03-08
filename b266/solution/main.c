/*
  Submission ID: 15613505
  AC (2ms, 76KB)
 */

#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

void swap(int *a, int *b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

/* The transform describes the operation T \dot R^k, where the k
   ranges from 0 to 3. */
typedef struct transform_t {
    int rotation;
    bool transpose;
} Transform;

/* Inverse a transform.

   If the transform has a transpose like T R^k, the inverse is the
   same with it self.

   Otherwise, if the transform is a series of rotations only like R^k,
   the inverse is R^(4-k mod 4).
 */
void transform_inverse(Transform *trans) {
    if (!trans->transpose) {
        trans->rotation = (4 - trans->rotation) % 4;
    }
}

/* Apply one transpose on the left, which may add or cancel a
   transpose on the left. */
void transform_transpose(Transform *trans) {
    trans->transpose = !trans->transpose;
}

/* Apply k rotations on the left.
   Consider two cases:

   If there is no transpose on the left, that is, the transform is
   R^n. It becomes R^((n + k) mod 4) because R^4 equals to an
   identity operation.

   If there is a transpose, use the equation R^(k%4) T = T
   R^(4-(k%4)).
 */
void transform_rotate(Transform *trans, int k) {
    if (trans->transpose) {
        trans->rotation = (trans->rotation + (4 - k % 4)) % 4;
    } else {
        trans->rotation = (trans->rotation + k) % 4;
    }
}

/* Perform on vertical flip. It is equivalent to a transpose and then
   three rotations. */
void transform_vertical_flip(Transform *trans) {
    transform_transpose(trans);
    transform_rotate(trans, 3);
}

int main() {
    /* Read the number of rows, columns and operations. */
    int nr, nc, m;
    scanf("%d %d %d", &nr, &nc, &m);

    /* Early exit if the number of elements is zero. */
    int ne = nr * nc;
    if (ne == 0) {
        return 0;
    }

    /* Read matrix data */
    int *data = (int *)malloc(ne * sizeof(int));
    assert(data != NULL);

    for (int i = 0; i < ne; i += 1) {
        scanf("%d", &data[i]);
    }

    /* Perform a series of rotation and verticle flips. */
    Transform trans = {
        .transpose = false,
        .rotation = 0,
    };

    for (int i = 0; i < m; i += 1) {
        int op;
        scanf("%d", &op);

        if (op == 0) {
            transform_rotate(&trans, 1);
        } else {
            transform_vertical_flip(&trans);
        }
    }

    /* Inverse the transform. */
    transform_inverse(&trans);

    /* Let F is the combined series of transformations and F^{-1} is
       the inverse. We know that A = F^{-1} \dot B.

       F^{-1} itself can be normalized to a 0~3 rotations plus an
       optional transpose R^k T. Hence, the original matrix A can be
       formulated as

       A = T R^k B

       To recover the matrix A, we don't actually rotate or tranpose
       the matrix B. Instead, we change the way we traverse the matrix
       B. The traversal can be described an initial position, a row
       step and a column step.

       The initial position describes the place of top-left element of
       A in the buffer.

       The column step describes how many ints to skip in the buffer
       so that the pointer moves from one cell in A to the cell in the
       next column.

       The row step describes how many ints to skip in the buffer so
       that the pointer moves from one cell in A to the cell in the
       next row.
    */
    int start;
    int row_step;
    int col_step;
    int row_count;
    int col_count;

    switch (trans.rotation) {
    case 0:
        start = 0;
        row_step = nc;
        col_step = 1;
        row_count = nr;
        col_count = nc;
        break;
    case 1:
        start = ne - nc;
        row_step = 1;
        col_step = -nc;
        row_count = nc;
        col_count = nr;
        break;
    case 2:
        start = ne - 1;
        row_step = -nc;
        col_step = -1;
        row_count = nr;
        col_count = nc;
        break;
    case 3:
        start = nc - 1;
        row_step = -1;
        col_step = nc;
        row_count = nc;
        col_count = nr;
        break;
    }

    if (trans.transpose) {
        swap(&row_step, &col_step);
        swap(&row_count, &col_count);
    }

    /* Traverse the buffer to print the recovered matrix A */
    printf("%d %d\n", row_count, col_count);

    int *row_begin = &data[start];
    int *row_end = &data[start + row_step * row_count];

    for (int *row = row_begin; row != row_end; row += row_step) {
        int *col_begin = row;
        int *col_end = row + col_step * col_count;

        printf("%d", *col_begin);

        for (int *col = col_begin + col_step; col != col_end; col += col_step) {
            printf(" %d", *col);
        }

        printf("\n");
    }

    /* Free allocated memory */
    free(data);

    return 0;
}
