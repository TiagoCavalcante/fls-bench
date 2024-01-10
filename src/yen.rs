use graphs::Graph;
use std::collections::BinaryHeap;

/// Breadth First Search.
/// Returns the shortest path between `start` and `end`.
/// Returns `None` if no path exists.
/// ```
/// let graph = graph::Graph::new(300, 0.01);
/// let path = path::bfs(&graph, 0, 299).unwrap_or(vec![]);
/// println!("{:?}", path);
/// ```
fn bfs(
  graph: &Graph,
  start: usize,
  end: usize,
) -> Option<Vec<usize>> {
  // A queue to maintain the vertices whose adjacency list
  // is to be scanned as per normal DFS algorithm.
  let mut queue = std::collections::VecDeque::new();

  // Here usize::MAX is used to indicate that there is no
  // predecessor.
  let mut predecessor = vec![usize::MAX; graph.size];

  queue.push_back(start);

  // Standard BFS algorithm
  // See https://en.wikipedia.org/wiki/Breadth-first_search.
  while let Some(current) = queue.pop_front() {
    for &vertex in graph.get_neighbors(current) {
      if predecessor[vertex] == usize::MAX
        && vertex != start
      {
        predecessor[vertex] = current;
        queue.push_back(vertex);

        // We stop the BFS when we find the destination.
        if vertex == end {
          let mut path = vec![end];
          let mut current = end;
          while predecessor[current] != usize::MAX {
            path.push(predecessor[current]);
            current = predecessor[current];
          }

          path.reverse();

          return Some(path);
        }
      }
    }
  }

  // Start and end are not connected.
  None
}

pub fn yen(
  graph: &mut Graph,
  start: usize,
  end: usize,
  length: usize,
) -> Option<Vec<usize>> {
  if let Some(shortest_path) = bfs(graph, start, end) {
    if shortest_path.len() == length {
      return Some(shortest_path);
    }

    if shortest_path.len() > length {
      return None;
    }

    let mut paths = vec![shortest_path];
    let mut candidates = BinaryHeap::new();

    for i in 1..usize::MAX {
      for j in 0..paths[i - 1].len() - 1 {
        let spur_node = paths[i - 1][j];
        let root_path = &paths[i - 1][0..j + 1];

        let mut filtered_edges = vec![];
        for path in &paths {
          if path.len() > j && root_path == &path[0..j + 1]
          {
            let a = path[j];
            let b = path[j + 1];

            if graph.has_edge(a, b) {
              graph.remove_edge(a, b);
              filtered_edges.push((a, b))
            }
          }
        }

        for &node in
          root_path.iter().take(root_path.len() - 1)
        {
          let neighbors = graph.get_neighbors(node).clone();
          for neighbor in neighbors {
            graph.remove_edge(node, neighbor);
            filtered_edges.push((node, neighbor));
          }
        }
        if let Some(spur_path) = bfs(graph, spur_node, end)
        {
          let mut total_path = vec![];

          for &node in
            root_path.iter().take(root_path.len() - 1)
          {
            total_path.push(node);
          }

          for &node in &spur_path {
            total_path.push(node);
          }

          candidates.push(total_path);
        }

        for (a, b) in filtered_edges {
          graph.add_edge(a, b);
        }
      }

      if let Some(path) = candidates.pop() {
        if path.len() == length {
          return Some(path);
        }

        if path.len() > length {
          return None;
        }

        paths.push(path);
      } else {
        break;
      }
    }
  }

  None
}
