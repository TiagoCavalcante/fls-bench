#![feature(test)]
extern crate test;

use graphs::Graph;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{fs::File, io::Write, time::Instant};

mod fls;
mod yen;

enum Algorithm {
  Yen,
  Fls,
}

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
  let end_vertex = 1;
  let start_length = 1;
  let end_length = 100;
  let run_amount = 10;
  let warm_up_iterations = 100;
  let seed = 79544948;

  let mut times = vec![];

  let mut graph = Graph::new(size);

  for i in 1..warm_up_iterations {
    graph.fill_undirected(
      density * (i / warm_up_iterations) as f32,
    );
    let _ = test::black_box(yen::yen(
      &mut graph,
      start_vertex,
      end_vertex,
      start_length,
    ));
    let _ = test::black_box(fls::fls(
      &graph,
      start_vertex,
      end_vertex,
      start_length,
    ));
    graph.clear();
  }

  let mut rng = StdRng::seed_from_u64(seed);

  for length in start_length..=end_length {
    let mut repeated_times = vec![];

    for _ in 0..run_amount {
      graph.fill_undirected(density);

      // List of algorithm names to shuffle.
      let mut algorithms =
        vec![Algorithm::Yen, Algorithm::Fls];

      let mut yen_time = 0.0;
      let mut fls_time = 0.0;

      // Randomize the order of algorithm execution.
      algorithms.shuffle(&mut rng);

      for algo in algorithms {
        let now = Instant::now();

        let path = match algo {
          Algorithm::Yen => test::black_box(yen::yen(
            &mut graph,
            start_vertex,
            end_vertex,
            length,
          )),
          Algorithm::Fls => test::black_box(fls::fls(
            &graph,
            start_vertex,
            end_vertex,
            length,
          )),
        };

        let elapsed = now.elapsed().as_secs_f32();

        if let Some(path) = path {
          check_path(
            &graph,
            &path,
            start_vertex,
            end_vertex,
            length,
          );

          match algo {
            Algorithm::Yen => yen_time = elapsed,
            Algorithm::Fls => fls_time = elapsed,
          }
        }
      }

      repeated_times.push((yen_time, fls_time));
      graph.clear();
    }

    // Calculate the average time.
    let (total_yen_time, total_fls_time) =
      repeated_times.iter().fold((0.0, 0.0), |acc, &x| {
        (acc.0 + x.0, acc.1 + x.1)
      });

    times.push((
      total_yen_time / repeated_times.len() as f32,
      total_fls_time / repeated_times.len() as f32,
    ));
  }

  let mut yen_times = File::create("./target/yen_times")?;
  let mut fls_times = File::create("./target/fls_times")?;

  for (yen, fls) in &times {
    yen_times.write_all(format!("{}\n", yen).as_bytes())?;
    fls_times.write_all(format!("{}\n", fls).as_bytes())?;
  }

  Ok(())
}
