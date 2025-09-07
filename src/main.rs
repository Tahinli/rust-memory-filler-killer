use std::{process::exit, thread::sleep, time::Duration};

use sysinfo::{Pid, System};

struct Input {
    control_delay: u16,
    dealloc_delay: u16,
    include_swap: bool,
    kill_threshold: f64,
}

fn main() {
    println!("Hello, world!");
    let input = get_parameters();
    let mut system = System::new_all();
    loop {
        sleep(Duration::from_millis(input.control_delay as u64));
        if let Some(memory_filler) = find_memory_filler(&mut system, &input) {
            kill_memory_filler(memory_filler, &mut system, &input)
        }
    }
}

fn find_memory_filler(system: &mut System, input: &Input) -> Option<Pid> {
    system.refresh_all();
    let used_memory = system.used_memory() as f64;
    let total_memory = if input.include_swap {
        (system.total_memory() + system.total_swap()) as f64
    } else {
        system.total_memory() as f64
    };
    if (used_memory / total_memory) >= input.kill_threshold {
        let mut memory_filler = system.process(Pid::from_u32(1)).unwrap();
        for (_, process) in system.processes() {
            if process.memory() > memory_filler.memory() {
                memory_filler = process;
            }
        }
        Some(memory_filler.pid())
    } else {
        None
    }
}

fn kill_memory_filler(memory_filler: Pid, system: &mut System, input: &Input) {
    let memory_filler = match system.process(memory_filler) {
        Some(process) => process,
        None => return,
    };
    println!(
        "Memory Filler = {} | Pid: {} | Used Memory: {} |\nUsed Memory Percentage: \n\tMemory -> %{}\n\tMemory + Swap -> %{}",
        memory_filler.name().to_string_lossy(),
        memory_filler.pid(),
        memory_filler.memory() / (1024 * 1024),
        memory_filler.memory() as f64 / system.total_memory() as f64,
        memory_filler.memory() as f64 / (system.total_memory() + system.total_swap()) as f64
    );
    memory_filler.kill();
    memory_filler.wait();
    sleep(Duration::from_millis(input.dealloc_delay as u64));
}

fn get_parameters() -> Input {
    let env_values = std::env::args().collect::<Vec<String>>();
    let mut input = Input {
        control_delay: 1000,
        dealloc_delay: 1000,
        include_swap: false,
        kill_threshold: 0.95,
    };
    let get_env_value = |env_values: &Vec<String>, index: usize| -> Option<String> {
        env_values.get(index).map(|env_value| env_value.to_string())
    };
    let parse_env_value_to_u16 =
        |env_value: Option<String>| env_value.map(|env_value| env_value.parse::<u16>());
    let parse_env_value_to_bool =
        |env_value: Option<String>| env_value.map(|env_value| env_value.parse::<bool>());
    let parse_env_value_between_zero_and_one = |env_value: Option<String>| {
        env_value.map(|env_value| {
            env_value.parse::<f64>().map(|parsed_value| {
                if parsed_value > 1.0 {
                    return 1.0;
                } else if parsed_value < 0.0 {
                    return 0.0;
                } else {
                    parsed_value
                }
            })
        })
    };
    for (i, env_value) in env_values.iter().enumerate() {
        match env_value.as_str() {
            "--control_delay" | "-cd" => {
                input.control_delay = parse_env_value_to_u16(get_env_value(&env_values, i + 1))
                    .unwrap_or(Ok(input.control_delay))
                    .unwrap_or(input.control_delay);
            }
            "--dealloc_delay" | "-dd" => {
                input.dealloc_delay = parse_env_value_to_u16(get_env_value(&env_values, i + 1))
                    .unwrap_or(Ok(input.dealloc_delay))
                    .unwrap_or(input.dealloc_delay);
            }
            "--include_swap" | "is" => {
                input.include_swap = parse_env_value_to_bool(get_env_value(&env_values, i + 1))
                    .unwrap_or(Ok(input.include_swap))
                    .unwrap_or(input.include_swap);
            }
            "--kill_threshold" | "kt" => {
                input.kill_threshold =
                    parse_env_value_between_zero_and_one(get_env_value(&env_values, i + 1))
                        .unwrap_or(Ok(input.kill_threshold))
                        .unwrap_or(input.kill_threshold);
            }
            "--help" | "-h" => {
                show_help();
                exit(0);
            }
            _ => {}
        }
    }
    input
}

fn show_help() {
    println!("\n\n\n");
    println!("Arguments              |  Details                          |  Defaults");
    println!("------------------------------------------------------------------------------");
    println!("-cd | --control_delay  |  Process Control Delay            |  1000(ms)");
    println!("-dd | --dealloc_delay  |  Dealloc Delay, After Termination |  1000(ms)");
    println!("-is | --include_swap   |  Include Swap for Total Memory    |  false(bool)");
    println!("-kt | --kill_threshold |  Memory Limit to Kill Process     |  0.95(0.0-1.0)");
    println!("-h  | --help           |  Shows Help");
    println!("\n\n\n");
}
