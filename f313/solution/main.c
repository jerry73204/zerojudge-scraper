#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* int get(int *buf, int r, int c) {} */

typedef struct grid {
  int *buf;
  int nc;
  int area;
} Grid;

void grid_init(Grid *grid, int nr, int nc) {
  int area = nr * nc;
  int *buf = (int*) malloc(sizeof(int) * area);
  assert(buf != NULL);

  grid->buf = buf;
  grid->nc = nc;
  grid->area = area;
}

int *grid_ptr(Grid *grid, int ir, int ic) {
  int ix = ir * grid->nc + ic;
  if (ir < 0 || ic < 0 || ic >= grid->nc || ix >= grid->area) {
    return NULL;
  }
  return &grid->buf[ix];
}

void grid_copy(Grid *src, Grid *dst) {
  assert(src->nc == dst->nc);
  assert(src->area == dst->area);
  memcpy(dst->buf, src->buf, sizeof(int) * src->area);
}

void grid_destroy(Grid *grid) {
  free(grid->buf);
}

void swap(Grid **ptr1, Grid **ptr2) {
  Grid *tmp = *ptr1;
  *ptr1 = *ptr2;
  *ptr2 = tmp;
}

int max(int a, int b) {
  if (a >= b) {
    return a;
  } else {
    return b;
  }
}

int min(int a, int b) {
  if (a <= b) {
    return a;
  } else {
    return b;
  }
}

int main() {
  /* Read R, C, k, m parameters */
  int nr, nc, k, m;
  scanf("%d %d %d %d", &nr, &nc, &k, &m);

  /* Initialize grids */
  Grid grid1;
  grid_init(&grid1, nr, nc);

  Grid grid2;
  grid_init(&grid2, nr, nc);

  Grid *curr = &grid1;
  Grid *next = &grid2;

  /* Fill the grid */
  for (int ir = 0; ir < nr; ir += 1) {
    for (int ic = 0; ic < nc; ic += 1) {
      scanf("%d", grid_ptr(curr, ir, ic));
    }
  }


  /* Repeat m rounds */
  for (int im = 0; im < m; im += 1) {
    grid_copy(curr, next);

    /* Update the numbers in the grid */
    for (int ir = 0; ir < nr; ir += 1) {
      for (int ic = 0; ic < nc; ic += 1) {

	/* Get the pointer to the source cell */
	int *src = grid_ptr(curr, ir, ic);

	/* If the cell is not a city (-1) or has no residents, skip
	   this cell. */
	if (*src == -1 || *src == 0) {
	  continue;
	}


	int transfer = *src / k;

	int *ccell = grid_ptr(next, ir, ic);

	int *lcell = grid_ptr(next, ir - 1, ic);
	if (lcell != NULL && *lcell != -1) {
	  *lcell += transfer;
	  *ccell -= transfer;
	}

	int *rcell = grid_ptr(next, ir + 1, ic);
	if (rcell != NULL && *rcell != -1) {
	  *rcell += transfer;
	  *ccell -= transfer;
	}

	int *bcell = grid_ptr(next, ir, ic - 1);
	if (bcell != NULL && *bcell != -1) {
	  *bcell += transfer;
	  *ccell -= transfer;
	}

	int *tcell = grid_ptr(next, ir, ic + 1);
	if (tcell != NULL && *tcell != -1) {
	  *tcell += transfer;
	  *ccell -= transfer;
	}
      }
    }

    swap(&curr, &next);
  }

  /* Find the maximum and minimum values in the grid */
  int min_val = INT_MAX;
  int max_val = INT_MIN;
  for (int ir = 0; ir < nr; ir += 1) {
    for (int ic = 0; ic < nc; ic += 1) {
      int val = *grid_ptr(curr, ir, ic);

      if (val == -1) {
	continue;
      }

      min_val = min(min_val, val);
      max_val = max(max_val, val);
    }
  }

  /* Print the min and max values */
  printf("%d\n", min_val);
  printf("%d\n", max_val);

  /* Finalization */
  grid_destroy(&grid1);
  grid_destroy(&grid2);

  return 0;
}
