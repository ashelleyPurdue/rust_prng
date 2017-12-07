use hashing;

pub struct RandGen
{
    seed: u64,
    state: u64
}

impl RandGen
{
    // Constructor
    pub fn new(seed: u64) -> RandGen
    {
        RandGen
        {
            seed: seed,
            state: seed
        }
    }

    // Methods
    pub fn next_u64(&mut self, max: u64) -> u64
    {
        let output = hashing::stack_overflow_hash(self.state);         // Hash the state for our output
        self.state = self.state.wrapping_add(output);   // Do something to the state to make
                                                        // the next output different.
        return output % max;
    }
}