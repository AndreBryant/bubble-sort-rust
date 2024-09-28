use std::time::SystemTime;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Data {
    size: i32,
    average: f64,
    trials: Vec<u128>
}

fn main() {
    let mut bubble_sort_data: Vec<Data> = vec![];
    let mut optimized_bubble_sort_data: Vec<Data> = vec![];
    let sizes = [100, 1000, 10000];
    let trials = 5;

    for size in &sizes {
        println!("Vector size: {}", size);
        let mut arr: Vec<i32> = (1..=*size).collect();
        arr.shuffle(&mut thread_rng());

        let mut bs_data = Data {
            size: 0,
            average: 0.0,
            trials: vec![]
        };

        let mut obs_data = Data {
            size: 0,
            average: 0.0,
            trials: vec![]
        };

        for _ in 0..trials {
            let mut sorted1 = arr.clone();
            {
                let start_time = SystemTime::now();
                bubble_sort(&mut sorted1);
                let elapsed = start_time.elapsed().expect("error idk yet").as_millis();
                bs_data.average += elapsed as f64;
                bs_data.trials.push(elapsed);
            }

            let mut sorted2 = arr.clone();
            {
                let start_time = SystemTime::now();
                optimized_bubble_sort(&mut sorted2);
                let elapsed = start_time.elapsed().expect("error idk yet").as_millis();
                obs_data.average += elapsed as f64;
                obs_data.trials.push(elapsed);
            }
        }
        bs_data.size = *size;
        bs_data.average /= trials as f64;
        bubble_sort_data.push(bs_data);

        obs_data.size = *size;
        obs_data.average /= trials as f64;
        optimized_bubble_sort_data.push(obs_data);
    }

    print_data(bubble_sort_data, "Bubble Sort Data");
    print_data(optimized_bubble_sort_data, "Optimized Bubble Sort Data");

}

fn bubble_sort(arr: &mut Vec<i32>){
    for _ in 0..arr.len()-1 {
        for j in 0..arr.len()-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn optimized_bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len()-1 {
        let mut swapped = false;
        for j in 0..arr.len()-1-i {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn print_data(data: Vec<Data>, title: &str) {
    println!("{}", title);
    println!("|{0: <15} | {1: <15}|", "Size", "Average (ms)");
    for d in &data {
        print_format((*d).size, (*d).average);
    }
}

fn print_format(size: i32, average: f64) {
     println!("|{0: <15} | {1: <15}|", size, average);
}