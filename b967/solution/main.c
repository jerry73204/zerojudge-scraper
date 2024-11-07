#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node {
  /* The parent node index */
  int parent;

  /* The number of children on the node */
  int child_count;

  /* The number of visits from children */
  int visits;

  /* Top 2 heights from two distinct children */
  int height1;
  int height2;

  /* The maximum distance within the subtree rooted on this node */
  int subtree_distance;
} Node;

int max(int a, int b) {
  if (a >= b) {
    return a;
  } else {
    return b;
  }
}

int main() {
  /* Read the number of people */
  int n;
  scanf("%d", &n);

  /* Initialize nodes */
  Node *nodes = (Node *)malloc(n * sizeof(Node));
  assert(nodes != NULL);

  for (Node *ptr = nodes; ptr != nodes + n; ptr += 1) {
    ptr->parent = -1;
    ptr->child_count = 0;
    ptr->visits = 0;
    ptr->height1 = INT_MIN;
    ptr->height2 = INT_MIN;
    ptr->subtree_distance = INT_MIN;
  }

  /* Read relations */
  for (int i = 0; i < n - 1; i += 1) {
    int parent, child;
    scanf("%d %d", &parent, &child);
    nodes[child].parent = parent;
  }

  /* Count the number of children on each node */
  for (Node *ptr = nodes; ptr != nodes + n; ptr += 1) {
    nodes[ptr->parent].child_count += 1;
  }

  /* The global max distance will be the answer given by this
     program. */
  int global_max_distance = INT_MIN;

  /* Iterate through node array to search for leaf nodes. If a leaf
     node is encoutered, start to walk from the leaf to parent and
     grandparent nodes. Update the heights and max distances on each
     parent node. */
  for (Node *start = nodes; start != nodes + n; start += 1) {
    /* Skip non-leaf nodes */
    if (start->child_count != 0) {
      continue;
    }

    /* Set 1st height and max distance to zero on the leaf
       node. */
    start->height1 = 0;
    start->subtree_distance = 0;

    /* Assert that the leaf node always has a parent. */
    assert(start->parent != -1);

    /* Prepare state varaibles to traverse from the immediate leaf
       parent to grandparent nodes. */
    Node *curr = &nodes[start->parent];
    int height = 1;
    int subtree_distance = 0;

    /* Visit a (grand-)parent in each loop. */
    while (1) {
      /* Increase the visit count on the parent node. */
      curr->visits += 1;

      /* Insert the height to the top 2 heights on the parent
         node. */
      if (height >= curr->height1) {
        curr->height2 = curr->height1;
        curr->height1 = height;
      } else if (height >= curr->height2) {
        curr->height2 = height;
      }

      /* Update the maximum distance within the subtree rooted
         in the parent node. */
      subtree_distance = max(curr->subtree_distance, subtree_distance);
      curr->subtree_distance = subtree_distance;

      /* If all of children nodes of this parent node are
         visited, compute the maximum distance passing through
         the parent node. If not, stop the traversal. */
      if (curr->visits == curr->child_count) {
        /* Compute the maximum distance passing through the
           parent node */
        int distance_thru_node;
        assert(curr->height1 != INT_MIN);

        if (curr->height2 != INT_MIN) {
          distance_thru_node = curr->height1 + curr->height2;
        } else {
          distance_thru_node = curr->height1;
        }

        /* Compare the distance with the max distance from
           children */
        subtree_distance = max(curr->subtree_distance, distance_thru_node);
        curr->subtree_distance = subtree_distance;

        /* Update the global maximum distance */
        global_max_distance = max(global_max_distance, subtree_distance);

        /* Continue traversal if this node has a parent. */
        if (curr->parent != -1) {
          height += 1;
          curr = &nodes[curr->parent];
          continue;
        }
      }

      break;
    }
  }

  /* Find the maximum distance within the tree */
  printf("%d\n", global_max_distance);

  /* Finalize */
  free(nodes);

  return 0;
}
