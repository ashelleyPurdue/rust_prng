mod random;

fn main()
{
    // Make a new RandGen
    let mut rand_gen = random::RandGen::new(1337);

    // Print 10 random numbers
    for _ in 0..10
    {
        println!("{}", rand_gen.next_u64(10));
    }
}
