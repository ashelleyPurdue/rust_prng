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