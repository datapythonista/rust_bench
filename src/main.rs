use std::time;
use std::env;
use core_affinity;
use perf_event;
use perf_event::events::{Hardware,Software};

pub mod bench;
pub mod prime;


fn run_benchmark_once() -> (u64, u128, u64, Vec<u64>) {
    let events_hw = [
        Hardware::CPU_CYCLES,
        Hardware::INSTRUCTIONS,
        Hardware::CACHE_REFERENCES,
        Hardware::CACHE_MISSES,
        Hardware::BRANCH_INSTRUCTIONS,
        Hardware::BRANCH_MISSES,
        Hardware::REF_CPU_CYCLES];
    let events_sw = [
        Software::CPU_CLOCK,
        Software::TASK_CLOCK,
        Software::PAGE_FAULTS,
        Software::CONTEXT_SWITCHES,
        Software::CPU_MIGRATIONS,
        Software::PAGE_FAULTS_MIN,
        Software::PAGE_FAULTS_MAJ];

    let mut group = perf_event::Group::new().unwrap();

    let mut events = Vec::new();

    for event in events_hw {
        events.push(perf_event::Builder::new()
                               .group(&mut group)
                               .kind(event)
                               .build()
                               .unwrap());
    }
    for event in events_sw {
        events.push(perf_event::Builder::new()
                               .group(&mut group)
                               .kind(event)
                               .build()
                               .unwrap());
    }

    let time_start = time::Instant::now();
    group.enable().unwrap();
    let rdtscp_start = bench::rdtscp();
    let result = prime::num_primes_until(3_000_000);
    let rdtscp_end = bench::rdtscp();
    group.disable().unwrap();
    let time_elapsed = time_start.elapsed();

    let counts = group.read().unwrap();

    (result, time_elapsed.as_millis(), rdtscp_end - rdtscp_start, events.iter().map(|x| counts[&x]).collect())
}

fn run_benchmark_many(iterations: u64) {
    println!("result,time_elapsed_ms,rdtscp,cpu_cycles,instructions,cache_references,cache_missed,\
              branch_instructions,branch_misses,ref_cpu_cycles,cpu_clock,\
              task_clock,page_faults,context_switches,cpu_migrations,\
              page_faults_min,page_faults_maj");
    for _ in 0..iterations {
        let (benchmark_result, time_elapsed, rdtscp, perf_results) = run_benchmark_once();
        print!("{},{},{}", benchmark_result, time_elapsed, rdtscp);
        for value in perf_results {
            print!(",{}", value);
        }
        print!("\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let iterations: u64 = {if args.len() >= 2 { args[1].parse().unwrap() } else { 1000 }};

    core_affinity::set_for_current(core_affinity::CoreId{id: 3});

    run_benchmark_many(iterations);
}
