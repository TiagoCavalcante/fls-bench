use graphs::Graph;
use std::{fs::File, io::Write, time::Instant};

mod fls;
mod yen;

fn check_path(
  graph: &Graph,
  path: &Vec<usize>,
  start: usize,
  end: usize,
  length: usize,
) {
  assert_eq!(path.len(), length);
  assert_eq!(*path.first().unwrap(), start);
  assert_eq!(*path.last().unwrap(), end);

  // Check if the path is made only by real edges.
  for index in 0..path.len() - 1 {
    assert!(graph.has_edge(path[index], path[index + 1]));
  }

  // Ensure that the path contain no loops.
  let mut unique = path.clone();
  // We need a sorted vector to use dedup.
  unique.sort();
  unique.dedup();
  // If the path had loops then the length of the unique
  // vector would be smaller than the length of the path.
  assert_eq!(path.len(), unique.len());
}

fn main() -> std::io::Result<()> {
  let size = 1_000;
  let density = 0.1;
  let start_vertex = 0;
  let end_vertex = 10;
  let start_length = 3;
  let end_length = 100;

  let mut times = vec![];

  let mut graph = Graph::new(size);

  for length in start_length..=end_length {
    let mut repeats = 0;
    let mut repeated_times = vec![];

    while repeats != 10 {
      graph.fill_undirected(density);

      let now = Instant::now();
      let yen_path = yen::yen(
        &mut graph,
        start_vertex,
        end_vertex,
        length,
      );
      let yen_time = now.elapsed().as_secs_f32();
      let now = Instant::now();

      let fls_path =
        fls::fls(&graph, start_vertex, end_vertex, length);
      let fls_time = now.elapsed().as_secs_f32();

      if let (Some(yen_path), Some(fls_path)) =
        (yen_path, fls_path)
      {
        check_path(
          &graph,
          &yen_path,
          start_vertex,
          end_vertex,
          length,
        );
        check_path(
          &graph,
          &fls_path,
          start_vertex,
          end_vertex,
          length,
        );

        repeated_times.push((yen_time, fls_time));

        repeats += 1;
      }

      graph.clear();
    }

    let time = repeated_times
      .iter()
      .fold((0.0, 0.0), |acc, current| {
        (acc.0 + current.0, acc.1 + current.1)
      });

    times.push((
      time.0 / repeats as f32,
      time.1 / repeats as f32,
    ));
  }

  let mut yen_times = File::create("./target/yen_times")?;
  let mut fls_times = File::create("./target/fls_times")?;

  for time in &times {
    yen_times.write(format!("{}\n", time.0).as_bytes())?;
    fls_times.write(format!("{}\n", time.1).as_bytes())?;
  }

  Ok(())
}
