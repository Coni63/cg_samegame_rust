use std::env;

use board::Board;

mod board;
mod input;
mod region;
mod solver;
mod solvers;

fn parse_options(args: &[String]) -> solver::Options {
    let mut opts = solver::Options::default();
    for arg in args {
        let (key, value) = arg.split_once('=').expect("options must be key=value");
        match key {
            "width" => opts.width = value.parse().unwrap(),
            "threads" => opts.threads = value.parse().unwrap(),
            "seed" => opts.seed = value.parse().unwrap(),
            "gw" => opts.group_weight = value.parse().unwrap(),
            "sp" => opts.single_penalty = value.parse().unwrap(),
            "tw" => opts.target_weight = value.parse().unwrap(),
            "ow" => opts.other_weight = value.parse().unwrap(),
            "nw" => opts.nested_width = value.parse().unwrap(),
            "init" => opts.from_db = value == "db",
            "noise" => opts.noise = value.parse().unwrap(),
            "targets" => opts.targets = value.split(',').map(|t| t.parse().unwrap()).collect(),
            _ => panic!("unknown option {}", key),
        }
    }
    opts
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // bench mode: solve several testcases without saving to the database
    // cg_samegame_rust bench testcases/test6.json,testcases/test7.json width=1000 gw=0.5
    if args[1] == "bench" {
        let opts = parse_options(&args[3..]);
        let mut total = 0;
        for file in args[2].split(',') {
            let testcase = input::load_json(file);
            let board = Board::new(testcase.board);
            let (_, score) = solver::solve(&board, &opts, None);
            println!("{} {}", testcase.title, score);
            total += score;
        }
        println!("total {}", total);
        return;
    }

    let testcase = input::load_json(&args[1]);
    let opts = parse_options(&args[2..]);

    let board = Board::new(testcase.board);
    eprintln!("{:?}", board);

    let known = if opts.from_db {
        input::load_best(testcase.hash).expect("cannot read the database")
    } else {
        None
    };
    let (actions, score) = solver::solve(&board, &opts, known.as_ref().map(|(a, _)| a.as_str()));
    println!("{}", actions);
    eprintln!("score: {}", score);

    match input::save_to_db(&testcase, &actions, score) {
        Ok(_) => eprintln!("Row inserted successfully!"),
        Err(e) => eprintln!("Error: {:?}", e),
    };
}
