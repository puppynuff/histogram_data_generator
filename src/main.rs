use std::time::Instant;
use std::env;

fn main() {
    let mut keep_var_values = false;
    let mut print_output = false;
    let mut bin_amount: f64 = 16384.0;

    let args: Vec<String> = env::args().collect();



    for i in 0..args.len() {
        if args[i] == "--keep_values" {
            keep_var_values = true;
        }

        if args[i] == "--print_output" {
            print_output = true;
        }

        if args[i] == "--bin_amount" {
            bin_amount = args[i+1].parse::<f64>().unwrap();
        }
    }

    println!("{:?}", bin_amount);

    println!("Creating Variables...");

    let mut random_vec: Box<[f64]> =Box::new([0.0; 100_000_000]);

    let mut now  = Instant::now();
    random_vec.iter_mut().enumerate().for_each(|(_, v)| {
        *v = rand::random();
    });
    let mut elapsed = now.elapsed();
    println!("Time elapsed using iter: {:?}", elapsed);

    println!("Finished making variables, now sorting them");

    // This needs to be 7x faster?
    now = Instant::now();
    let (int_data, full_data) = seperate_into_bins_vec(random_vec, bin_amount, keep_var_values);
    elapsed = now.elapsed();
    println!("Vector time elapsed is {:?}", elapsed);


    if keep_var_values == false && print_output == true {
        int_data.iter().enumerate().for_each(|(i, v)| {
            println!("Bin {} has {} items", i + 1, v);
        })
    }

    if keep_var_values == true && print_output == true {
        println!("{:?}", full_data);
    }


    println!("Finished!");
}


fn seperate_into_bins_vec(variables: Box<[f64]>, bin_amount: f64, save_values: bool) ->  (Box<[i64]>, Vec<Vec<f64>>) {
    if save_values {
        let mut bins: Vec<Vec<f64>> = vec![vec![]; bin_amount as usize];

        variables.iter().enumerate().for_each(|(_i, v)| {
            bins[(v * bin_amount as f64).floor() as usize].push(*v);
        });
        return (Box::new([]), bins);
    }

    let mut bins: Box<[i64]> = vec![0; bin_amount as usize].into();

    variables.iter().enumerate().for_each(|(_i, v)| {
        bins[(v * bin_amount as f64).floor() as usize] += 1;
    });
    return (bins, vec![]);

}
