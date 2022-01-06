use std::{
    env,
    process::exit,
    sync::{Arc, MutexGuard},
};
use tokio::{self, time::Instant};
// use mutex
use std::sync::Mutex;

struct Philo {
    n: u8,
    times: u32,
    t_eat: u64,
    t_sleep: u64,
    last_eat: Instant,
    t0: Instant,
}

impl Philo {
    fn new(n: u8, table: &Table) -> Self {
        Philo {
            n,
            last_eat: Instant::now(),
            times: 0,
            t_eat: table.t_eat,
            t_sleep: table.t_sleep,
            t0: Instant::now(),
        }
    }

    fn get_time(&self) -> u64 {
        let now = Instant::now();
        let d = now.duration_since(self.t0);
        d.as_secs() * 1000 + d.subsec_millis() as u64
    }

    async fn eat(&mut self) {
        println!("{}\t{} is eating", self.get_time(), self.n + 1);
        tokio::time::sleep(tokio::time::Duration::from_millis(self.t_eat)).await;
        self.times += 1;
    }

    async fn sleep(&self) {
        println!("{}\t{} is sleeping", self.get_time(), self.n + 1);
        tokio::time::sleep(tokio::time::Duration::from_millis(self.t_sleep)).await;
    }

    fn think(&self) {
        println!("{}\t{} is thinking", self.get_time(), self.n + 1);
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
                    return;
                }
                return;
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
