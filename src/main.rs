use std::{
    env,
    process::exit,
    sync::{Arc, MutexGuard},
};
use tokio::{self};
// use mutex
use std::sync::Mutex;

struct Philo {
    n: u8,

    times: u32,
    t_eat: u64,
    t_sleep: u64,
}

impl Philo {
    fn new(n: u8, table: &Table) -> Self {
        Philo {
            n,

            times: 0,
            t_eat: table.t_eat,
            t_sleep: table.t_sleep,
        }
    }

    async fn eat(&mut self) {
        println!("{} is eating", self.n);
        tokio::time::sleep(tokio::time::Duration::from_millis(self.t_eat)).await;
        println!("{} is done eating", self.n);
        self.times += 1;
    }

    async fn sleep(&self) {
        println!("{} is sleeping", self.n);
        tokio::time::sleep(tokio::time::Duration::from_millis(self.t_sleep)).await;
        println!("{} is done sleeping", self.n);
    }

    fn think(&self) {
        println!("{} is thinking", self.n);
        println!("{} is done thinking", self.n);
    }
}

struct Table {
    n: u8,
    t_die: u64,
    t_sleep: u64,
    t_eat: u64,
    n_times: u32, // number of times to eat (infinite if not provided)
    n_full: u32,  // number of philosophers that have eaten all times
}

impl Table {
    fn new(n_philosophers: u8, n_times: u32, t_die: u64, t_sleep: u64, t_eat: u64) -> Table {
        Table {
            n: n_philosophers,
            t_die,
            t_sleep,
            t_eat,
            n_times,
            n_full: 0,
        }
    }
}

async fn philo(table: Arc<Mutex<Table>>, n: u32) {
    let mut p = Philo::new(n as u8, &table.lock().unwrap());
    loop {
        p.sleep().await;
        p.eat().await;
        p.think();
        {
            let mut lock: MutexGuard<Table> = table.lock().unwrap();
            if p.times == lock.n_times {
                lock.n_full += 1;
                if lock.n_full == lock.n.into() {
                    println!("All philosophers have eaten {} times", lock.n_times);
                    exit(0);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 || args.len() > 6 {
        println!(
            "usage: {} <number of philosophers> <t_die> <t_eat> <t_sleep> [n_times!]",
            args[0]
        );
        exit(1);
    }
    for arg in &args[1..] {
        if arg.parse::<u32>().is_err() || arg.parse::<u32>().unwrap() < 1 {
            println!(
                "usage: {} <number of philosophers> <t_die> <t_eat> <t_sleep> [n_times]",
                args[0]
            );
            exit(1);
        }
    }
    let table = Arc::new(Mutex::new(Table::new(
        args[1].parse().unwrap(),
        if args.len() == 6 {
            args[5].parse().unwrap()
        } else {
            std::u32::MAX
        },
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
    )));
    let mut philos = vec![];
    for i in 0..table.lock().unwrap().n {
        philos.push(tokio::spawn(philo(table.clone(), i.into())));
    }
    for philo in philos {
        philo.await.unwrap();
    }
}
