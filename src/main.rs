//a program that generates a large number of random numbers then calculates the mean, median, and mode. 
use rand::Rng;


fn main() {
    let data_size: i32 = 10_000;
    let data_set: Vec<i32> = gen_numbers(data_size);
    let median: i32 = find_median(data_set.clone());
    println!("Median: {median}");
    let mean: i32 = find_mean(data_set.clone());
    println!("Mean: {mean}");
    let mode: i32 = find_mode(data_set.clone());
    println!("Mode: {mode}");
}

fn gen_numbers(data_size: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _i in 0..data_size {
        let x: i32 = rng.gen_range(0..10001);
        numbers.push(x)
    }

    /* for i in numbers.clone() {
        println!("{i}");
    } */
    return numbers;
}

fn find_median(data: Vec<i32>) -> i32 {
    //sort the list into numeric order
    let mut sorted: Vec<i32> = Vec::new();
    for (ind, elm) in data.iter().enumerate() {
        //put the first thing into sorted
        if ind == 0 {
            sorted.push(*elm)
        } else {
            let other_sorted = sorted.clone();
            for (index, element) in other_sorted.iter().enumerate() {
                if elm < element {
                    sorted.insert(index, *elm);
                    break;
                } else {
                    if index == sorted.len() - 1 {
                        sorted.push(*elm);
                        break;
                    }
                }
            }
        }
    }

    //now actually calculate the median
    if sorted.clone().len() % 2 == 0 {
        //stupid even numbers, just gotta average the middle two
        return (sorted[sorted.len()/2] + sorted[sorted.len()/2 - 1])/2
    } else {
        return sorted[sorted.len()/2];
    }

}


fn find_mean(dataset: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for i in dataset.clone() {
        total+=i;
    }
    return total/(dataset.len() as i32);
}

fn find_mode(dataset: Vec<i32>) -> i32 {
    let mut top_count = 1;
    let mut top_number = -1;
    let dataset2 = dataset.clone();
    for i in dataset.clone() {
        let checking = i;
        let mut count = 0;
        for j in &dataset2 {
            if checking == *j {
                count += 1;
            }
        }
        if count > top_count {
            top_number = i;
            top_count = checking;
        }
    }
    return top_number;
}


