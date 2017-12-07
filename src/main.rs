mod random;
mod hashing;

use random::RandGen;
use random::RandGenStackOverflow;

fn main()
{
    // Make a new RandGen
    let mut rand_gen = RandGenStackOverflow::new(1337);

    // Print its frequency graph
    print_frequency_graph(&mut rand_gen, 10, 100);
}

fn print_frequency_graph(rand_gen: &mut RandGen, max_num: u64, num_trials: u64)
{
    // Generates (num_trials) random numbers between 0 and (max_num)
    // Prints out a frequency graph

    let mut frequencies = vec![0; max_num as usize];

    // Generate all the numbers and tally their frequencies
    for _ in 0..num_trials
    {
        let num = rand_gen.next_u64(max_num) as usize;
        frequencies[num] += 1;
    }

    // Print the graph
    for num in 0..max_num
    {
        // Print which number this is
        print!("{}:\t", num);

        // Print a star for each time it appeared
        for _ in 0..frequencies[num as usize]
        {
            print!("*");
        }

        // Finish the line
        println!("");
    }
}
