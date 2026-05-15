use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Graph = Vec<Vec<(usize, u64)>>;

fn dijkstra(graph: &Graph, source: usize) -> Vec<u64> {
    let n = graph.len();
    let mut dist = vec![u64::MAX; n];
    dist[source] = 0;

    //reverse for min-heap
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u64, source)));

    while let Some(Reverse((cost, u))) = heap.pop() {
        if cost > dist[u] {
            continue;
        }

        for &(v, weight) in &graph[u] {
            let next_cost = dist[u] + weight;
            if next_cost < dist[v] {
                dist[v] = next_cost;
                heap.push(Reverse((next_cost, v)));
            }
        }
    }
    dist
}

fn main() {
    // Build a graph with 5 nodes (0–4)
    //
    //   0 --1-- 1
    //   |       |
    //   4       2
    //   |       |
    //   3 --1-- 2
    //    \     /
    //      5  3
    //       4
    //
    let mut graph: Graph = vec![vec![]; 5];

    let edges = vec![
        (0, 1, 1u64),
        (0, 3, 4),
        (1, 2, 2),
        (1, 3, 5),
        (2, 4, 3),
        (3, 4, 1),
    ];

    for (u, v, w) in edges {
        graph[u].push((v, w));
        graph[v].push((u, w)); // undirected
    }

    let source = 0;
    let distances = dijkstra(&graph, source);

    for (node, dist) in distances.iter().enumerate() {
        println!("Node {source} → Node {node}: {dist}");
    }
}
