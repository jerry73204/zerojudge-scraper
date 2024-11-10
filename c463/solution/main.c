#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node {
    int parent;
    int children_count;
    int visit_count;
    int height;
} Node;

int max(int a, int b) {
    if (a >= b) {
        return a;
    } else {
        return b;
    }
}

int main() {
    /* Read the number of nodes */
    int n;
    scanf("%d", &n);

    /* Initialize the node array. */
    Node *nodes = (Node *)malloc(n * sizeof(Node));
    assert(nodes != NULL);

    for (Node *ptr = nodes; ptr != nodes + n; ptr += 1) {
        ptr->parent = -1;
        ptr->children_count = 0;
        ptr->visit_count = 0;
        ptr->height = -1;
    }

    /* Read the parent-child relations */
    for (int parent = 0; parent < n; parent += 1) {
        int c;
        scanf("%d", &c);

        for (int i = 0; i < c; i += 1) {
            int child;
            scanf("%d", &child);
            child -= 1;
            nodes[child].parent = parent;
            nodes[parent].children_count += 1;
        }
    }

    /* Initialize the total subtree height to zero. */
    int total_height = 0;

    /* Scan for leaf nodes in the node array. Whenever a leaf node is
       found, start to traverse from the leaf to the root. */
    for (Node *start = nodes; start != nodes + n; start += 1) {
        /* Skip if the current node is not a leaf. */
        if (start->children_count != 0) {
            continue;
        }

        /* Start traversal from the leaf to its (grand-)parents in an
           inner loop. */
        int height = 0;
        Node *node = start;
        node->height = 0;

        while (1) {
            /* If the current node has any child not is not visited
               yet, stop the traversal and find the next leaf in the
               outer loop. */
            if (node->visit_count != node->children_count) {
                break;
            }

            /* Add the node height to the total. */
            total_height += node->height;

            if (node->parent == -1) {
                /* If this node does not have a parent, that is, the node
                   is a root. Since all children of this root are visited,
                   we print the results here and finish the program. */
                int root = (node - nodes) + 1;
                printf("%d\n%d\n", root, total_height);
                return 0;
            } else {
                /* Visit the node's parent. Increase the parent's
                   visit conut by 1 and update its height. */
                node = &nodes[node->parent];
                node->visit_count += 1;
                node->height = max(node->height, height + 1);
                height = node->height;
            }
        }
    }

    /* This place is not reachable because a root node will be finally
       found in the loop. */
    return 1;
}
