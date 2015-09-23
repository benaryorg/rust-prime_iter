//! This crate provides only one struct, `PrimeIter` implementing `Iterator`
//! which yields prime numbers as an `u64`.
//!

/// `PrimeIter` is used for generating an endless stream of prime numbers in an
/// efficient way.
///
/// The implementation of `Iterator` makes it easy to iterate over prime
/// numbers.
///
/// Generating the first ten prime numbers:
///
/// ```
/// # use primeiter::PrimeIter;
///
/// let primes = PrimeIter::new();
/// let v: Vec<_> = primes.take(10).collect();
///
/// assert_eq!(v,vec![2,3,5,7,11,13,17,19,23,29]);
/// ```
///
/// Printing an endless stream of prime numbers:
///
/// ```no_run
/// # use primeiter::PrimeIter;
/// for prime in PrimeIter::new()
/// {
///		println!("{}",prime);
/// }
/// ```
///
pub struct PrimeIter
{
	primes: Vec<u64>,
}

impl PrimeIter
{
	/// Returns a new instance of `PrimeIter`.
	///
	/// # Examples
	///
	/// ```
	/// # use primeiter::PrimeIter;
	/// let iter = PrimeIter::new();
	/// ```
	///
	pub fn new()->PrimeIter
	{
		PrimeIter
		{
			primes: Vec::new(),
		}
	}
}

impl Iterator for PrimeIter
{
	type Item = u64;

	fn next(&mut self)->Option<Self::Item>
	{
		if self.primes.last()==None
		{
			self.primes.push(2);
			return Some(2);
		}
		'outer: for num in ((self.primes.last().unwrap()+1)..).filter(|i|*i%2!=0)
		{
			if self.primes.iter().take_while(|i|*i**i<=num).any(|i|num%i==0)
			{
				continue 'outer;
			}
			self.primes.push(num);
			return Some(num);
		}
		None
	}
}

