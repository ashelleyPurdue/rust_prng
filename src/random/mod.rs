mod rand_gen_stack_overflow;
mod rand_gen_mod_exp;

use hashing;
pub use self::rand_gen_stack_overflow::RandGenStackOverflow;
pub use self::rand_gen_mod_exp::RandGenModExp;

pub trait RandGen
{
    fn next_u64(&mut self, max: u64) -> u64;
}

// TODO: How can I eliminate the duplicate code between both of these implementations?
