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

    // Associated functions(static methods in Java/C#)
    fn hash(num: u64) -> u64
    {
        // Returns the hash of the given number
        // Pilfered from stack overflow:
        // https://stackoverflow.com/questions/664014/what-integer-hash-function-are-good-that-accepts-an-integer-hash-key

        let mut h = num;

        h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        h = (h ^ (h >> 27)).wrapping_mul(0x94d049bb133111eb);
        h = h ^ (h >> 31);

        return h;
    }

    // Methods
    pub fn next_u64(&mut self, max: u64) -> u64
    {
        let output = RandGen::hash(self.state);         // Hash the state for our output
        self.state = self.state.wrapping_add(output);   // Do something to the state to make
                                                        // the next output different.
        return output % max;
    }
}