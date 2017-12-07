use hashing;

pub trait RandGen
{
    fn next_u64(&mut self, max: u64) -> u64;
}

mod rand_gen_stack_overflow
{
    use super::RandGen;
    use super::hashing;

    pub struct RandGenStackOverflow
    {
        seed: u64,
        state: u64
    }

    impl RandGenStackOverflow
    {
        // Constructor
        pub fn new(seed: u64) -> RandGenStackOverflow
        {
            RandGenStackOverflow
            {
                seed: seed,
                state: seed
            }
        }
    }

    impl RandGen for RandGenStackOverflow
    {
        fn next_u64(&mut self, max: u64) -> u64
        {
            let output = hashing::stack_overflow_hash(self.state);         // Hash the state for our output
            self.state = self.state.wrapping_add(output);   // Do something to the state to make
                                                            // the next output different.
            return output % max;
        }
    }
}

pub struct RandGenModExp
{
    seed: u64,
    state: u64
}

impl RandGenModExp
{
    // Constructor
    pub fn new(seed: u64) -> RandGenModExp
    {
        RandGenModExp
        {
            seed: seed,
            state: seed
        }
    }
}

impl RandGen for RandGenModExp
{
    fn next_u64(&mut self, max: u64) -> u64
    {
        let output = hashing::mod_exp_hash(self.state);         // Hash the state for our output
        self.state = self.state.wrapping_add(output);   // Do something to the state to make
                                                        // the next output different.
        return output % max;
    }
}