use bar_race::run_race;

const NUM_THREADS: usize = 5;
const MAX_COUNT: u32 = 100;

fn main() {
    println!("Welcome To the Thread Race Game!");
    println!(
        "There are {} threads racing to complete their task.",
        NUM_THREADS
    );

    println!("Starting the race ...");

    let winner = run_race(NUM_THREADS, MAX_COUNT);

    println!("Thread {} wins the race!", winner);
}
