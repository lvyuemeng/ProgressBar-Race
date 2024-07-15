use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Winner = Arc<Mutex<Option<usize>>>;
type ArcBar = Arc<ProgressBar>;
type ArcMultiBar = Arc<MultiProgress>;

fn thread_bar(id: usize, max_count: u32, pb: ArcBar) -> usize {
    let mut rng = rand::thread_rng();
    let mut count = 0;
    while count < max_count {
        let incr = rng.gen_range(1..10);
        let delay = rng.gen_range(1..=50);
        pb.inc(incr);
        count += incr as u32;
        thread::sleep(Duration::from_millis(delay as u64));
    }
    pb.finish_with_message("Done");
    id
}

fn run_race_wrapper(winner: Winner, num_threads: usize, max_count: u32, mp: ArcMultiBar) -> usize {
    (0..num_threads).into_par_iter().for_each(|id| {
        let pb = mp.add(ProgressBar::new(max_count as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "[{{elapsed_precise}}] {{bar:40.cyan/blue}} Thread {}: {{pos}}/{{len}} {{msg}}",
                    id
                ))
                .unwrap()
                .progress_chars("##-"),
        );

        let pb = Arc::new(pb);
        let result = thread_bar(id, max_count, pb);

        let mut winner_guard = winner.lock().unwrap();
        if winner_guard.is_none() {
            *winner_guard = Some(result);
        }
    });

    winner.lock().unwrap().unwrap()
}

pub fn run_race(num_threads: usize, max_count: u32) -> usize {
    let winner: Winner = Arc::new(Mutex::new(None));
    let mp = Arc::new(MultiProgress::new());
    run_race_wrapper(winner, num_threads, max_count, mp)
}
