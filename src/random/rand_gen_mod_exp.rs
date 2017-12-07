use super::RandGen;
use super::hashing;

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