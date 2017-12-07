mod random;
mod hashing;

use random::RandGen;

fn main()
{
    // Make a new RandGen
    let mut rand_gen = RandGen::new(1337);

    // Print 10 random numbers
    for _ in 0..10
    {
        println!("{}", rand_gen.next_u64(10));
    }
}
