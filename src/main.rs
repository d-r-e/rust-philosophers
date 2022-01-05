use std::{env, process::exit};
use tokio::sync;

struct philosopher {
    n: u8,
    left: sync::Mutex<u8>,
    right: sync::Mutex<u8>,
}

struct Table {
    forks: Vec<sync::Mutex<()>>,
    t_die: u32,
    t_sleep: u32,
    t_eat: u32,
    n_times: u32, // number of times to eat (infinite if not provided)
    n_full: u8, // number of philosophers that have eaten all times
}

impl Table {
    fn new(n_philosophers: u8, n_times: u32, t_die: u32, t_sleep: u32, t_eat: u32) -> Table {
        let forks: Vec<sync::Mutex<()>> = (0..n_philosophers).map(|_| sync::Mutex::new(())).collect();
        Table {
            forks,
            t_die,
            t_sleep,
            t_eat,
            n_times,
            n_full: 0,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 || args.len() > 6 {
        println!("usage: {} <number of philosophers> <t_die> <t_eat> <t_sleep> [n_times]", args[0]);
        exit(1);
    }
    for arg in &args {
        if arg.parse::<u32>().is_err() || arg.parse::<u32>().unwrap() < 1 {
            println!("usage: {} <number of philosophers> <t_die> <t_eat> <t_sleep> [n_times]", args[0]);
            exit(1);
        }
    }
    let table = Table::new(
        args[1].parse().unwrap(),
        if args.len() == 6 { args[5].parse().unwrap() } else { std::u32::MAX },
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
    );

    println!("{} philosophers, {} forks, {} seconds to die, {} seconds to eat, {} seconds to sleep", args[1], table.forks.len(), table.t_die, table.t_eat, table.t_sleep);
}
