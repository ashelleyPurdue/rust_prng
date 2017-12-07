const LARGEST_U64_PRIME: u64 = 2305843009213693951;		// Equivalent to 61 straight 1's in binary

pub fn stack_overflow_hash(num: u64) -> u64
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

pub fn mod_exp_hash(num: u64) -> u64
{
	// Returns a hash computed via modular exponentiation.
	// This is better than the stack overflow one, because it can't be
	// easily undone.
	// Unfortunately, it has a complexity of O(ln(n)), which sucks.

	const base: u64 = 5009424631521124603;	// A huge randomly-chose prime.
											// Insert xkcd "random" comic here
	
	return mod_exp(base, num, LARGEST_U64_PRIME);
}

fn mod_exp(base: u64, exp: u64, modulo: u64) -> u64
{
	// Returns base raised to the power of exp (mod modulo)
	// Algorithm pilfered from wikipedia:
	// https://en.wikipedia.org/wiki/Modular_exponentiation

	let mut result: u64 = 1;	
	let mut base: u64 = base % modulo;	// <3 variable shadowing :)
	let mut exp: u64 = exp;

	while exp > 0
	{
		if exp % 2 == 1
		{
			result = result.wrapping_mul(base) % modulo;
		}

		exp = exp >> 1;
		base = base.wrapping_mul(base) % modulo;
	}

	return result;
}