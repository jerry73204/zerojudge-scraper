#include <stdio.h>
#include <stdlib.h>

#define MAX_N 50
#define MAX_M 50
#define MAX_K 50

// Calculate the cost for sending a specific amount of traffic between cities
long long calculateCost(long long traffic, int is_same_city) {
    if (is_same_city) {
        return traffic; // Cost is 1 per unit if within the same city
    } else {
        if (traffic <= 1000) {
            return traffic * 3; // Cost is 3 per unit for traffic <= 1000
        } else {
            return 1000 * 3 + (traffic - 1000) * 2; // Cost is 2 per unit for traffic > 1000
        }
    }
}

int main() {
    int n, m, k;
    int Q[MAX_N][MAX_M]; // Traffic from server i to city j
    int placements[MAX_K][MAX_N]; // Placement strategies
    
    // Read input
    scanf("%d %d %d", &n, &m, &k);
    
    // Read traffic data
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            scanf("%d", &Q[i][j]);
        }
    }
    
    // Read placement strategies
    for (int i = 0; i < k; i++) {
        for (int j = 0; j < n; j++) {
            scanf("%d", &placements[i][j]);
        }
    }
    
    long long min_cost = -1; // Initialize to a large value
    
    // Evaluate each placement strategy
    for (int strategy = 0; strategy < k; strategy++) {
        long long total_cost = 0;
        
        // Initialize traffic matrix between cities
        long long city_traffic[MAX_M][MAX_M] = {0};
        
        // Calculate traffic flows between cities for this strategy
        for (int server = 0; server < n; server++) {
            int server_city = placements[strategy][server]; // City where server is placed
            
            // Add the traffic from this server to each destination city
            for (int dest_city = 0; dest_city < m; dest_city++) {
                city_traffic[server_city][dest_city] += Q[server][dest_city];
            }
        }
        
        // Calculate the cost for each city-to-city traffic
        for (int from_city = 0; from_city < m; from_city++) {
            for (int to_city = 0; to_city < m; to_city++) {
                if (city_traffic[from_city][to_city] > 0) {
                    total_cost += calculateCost(city_traffic[from_city][to_city], from_city == to_city);
                }
            }
        }
        
        // Update minimum cost
        if (min_cost == -1 || total_cost < min_cost) {
            min_cost = total_cost;
        }
    }
    
    // Output the minimum cost
    printf("%lld", min_cost);
    
    return 0;
}