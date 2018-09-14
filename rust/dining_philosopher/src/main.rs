use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}


impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        thread::sleep_ms(1000);
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{}은(는) 포크를 들었습니다.", self.name);

        println!("{}은(는) 밥을 먹었습니다.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("이난호", 0, 1),
        Philosopher::new("온성찬", 1, 2),
        Philosopher::new("양창선", 2, 3),
        Philosopher::new("봉은석", 3, 4),
        Philosopher::new("강석렬", 4, 0),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p|{
        let tmp_table = table.clone();

        thread::spawn(move || {
            p.eat(&tmp_table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
