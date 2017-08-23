extern crate num;
extern crate num_traits;

use num::bigint::BigInt;
use num_traits::{Zero, One};
use num::bigint::ToBigInt;

fn main() {
	assert_eq!(pe1(), 233168);
	assert_eq!(pe2(4000000), 4613732);
	assert_eq!(pe3(), 6857);
	assert_eq!(pe4_(), 906609);
//	assert_eq!(pe5(), 232792560); //slow, not so much in release!
	assert_eq!(pe6(), 25164150);
	assert_eq!(pe7(), 104743);
	assert_eq!(pe8().to_str_radix(10), String::from("23514624000"));
	assert_eq!(pe9(), 31875000);
//	assert_eq!(pe10(), 142913828922); // slow
	println!("pe1 {}", pe1());
	println!("pe2 {}", pe2(4000000));
	println!("pe8 {}", pe8());
	//println!("pe4 {}", pe4());
	//println!("pe4_ {}", pe4_());
	//println!("pe6 {}", pe6());
	//println!("pe9 {}", pe9());
	//println!("pe10 {}", pe10());
}

// PE1: find the sum of all the multiples of 3 or 5 below 1000
// 233168
fn pe1() -> i32 {
	(1..1000)
	.filter(|x| x % 3 == 0 || x % 5 == 0)
	.sum()
}


// PE2: terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms
// 4613732
fn fib(c: u64, n: u64, acc: u64, limit:u64) -> u64 {
	if n > limit {acc}
	else if n % 2 == 0 { fib(n, c + n, acc + n, limit) }
	else { fib(n, c + n, acc, limit) }
}

fn pe2(limit: u64) -> u64 { // limit = 4000000
	fib(1,2,0, limit)
}


// PE3: What is the largest prime factor of 600851475143
// 6857

// is x divisible by anthing in xs
fn has_divisor(x: u64, xs: &Vec<u64>) -> bool {
	xs.iter().any(|y| x % y == 0)
}

// make sure to call with at least one odd prime already added!
fn add_next_prime(primes_found: &mut Vec<u64>) {
	let mut next = *primes_found.last().unwrap() + 1;
	while has_divisor(next, &primes_found) {
		next += 1;
	};
	primes_found.push(next);
}

fn pe3() -> u64 {
	// accumulate a list of primes. Use them to look for the next prime,
	// which is next number not divisible by any of them
	let mut primes: Vec<u64> = vec![2];
	let mut divide_me:u64 = 600851475143;
	let mut next_prime:u64;

	loop {
		next_prime = *primes.last().unwrap();
		while divide_me % next_prime == 0 {
			divide_me = divide_me / next_prime;
		}
		if divide_me == 1 {
			break; // should be done
		}
		else {
			add_next_prime(&mut primes);
		}
	}
	next_prime
}

// PE4: Find the largest palindrome made from the product of two 3-digit numbers
// 906609
fn is_palindrome(x: i32) -> bool {
	let str:String = x.to_string();
	let rev:String = str.chars().rev().collect();
	str == rev
}

fn pe4_() -> i32 {
	(1..1000).fold(1, |accx,x|
		(x..1000).fold(accx, |accy, y| {
			let z = x * y;
			if z > accy && is_palindrome(z) {z} else {accy}
		})
	)
}

fn pe4() -> i32 {
	let mut largest = 1;
	for x in 1..1000 {
		for y in x..1000 {
			let z = x * y;
			if z > largest && is_palindrome(z) {
				largest = z;
			}
		}
	}
	largest
}


// PE5: What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20
// 232792560
fn divisible_by_all(x: i64, xs: &Vec<i64>) -> bool {
	xs.iter().all(|y| x % y == 0)
}

fn pe5() -> i64 {
	// no need to have numbers less than 11, if divisible by 20 also divisible by 10 duh.
	let nums: Vec<i64> = (11..21).rev().collect::<Vec<i64>>();
	let mut i:i64 = 1;
	while !divisible_by_all(i, &nums) {
		i += 1;
	}
	i
}


// PE6: square of sum minus sum of square, of 1..100
// 25164150
fn pe6() -> u64 {
	let sum: u64 =(1..101).sum();
	let square_of_sum = sum * sum;
	let sum_of_squares: u64 = (1..101).map(|x| x * x).sum();
	square_of_sum - sum_of_squares
}


// pe7 What is the 10001st prime number?
fn pe7() -> u64 {
	let mut primes = vec![2,3,5,7,11];
	while primes.len() < 10001 {
		add_next_prime(&mut primes);
	}
	*primes.last().unwrap()
}

// pe 8 thirteen adjacent digits in the 1000-digit number that have the greatest product
fn pe8() -> BigInt {
	let num_string = "73167176531330624919225119674426574742355349194934\
	96983520312774506326239578318016984801869478851843\
	85861560789112949495459501737958331952853208805511\
	12540698747158523863050715693290963295227443043557\
	66896648950445244523161731856403098711121722383113\
	62229893423380308135336276614282806444486645238749\
	30358907296290491560440772390713810515859307960866\
	70172427121883998797908792274921901699720888093776\
	65727333001053367881220235421809751254540594752243\
	52584907711670556013604839586446706324415722155397\
	53697817977846174064955149290862569321978468622482\
	83972241375657056057490261407972968652414535100474\
	82166370484403199890008895243450658541227588666881\
	16427171479924442928230863465674813919123162824586\
	17866458359124566529476545682848912883142607690042\
	24219022671055626321111109370544217506941658960408\
	07198403850962455444362981230987879927244284909188\
	84580156166097919133875499200524063689912560717606\
	05886116467109405077541002256983155200055935729725\
	71636269561882670428252483600823257530420752963450";

	let mut nums: [u32; 1000] = [0; 1000];

	for (i, c) in num_string.chars().enumerate() {
		nums[i] = c.to_digit(10).unwrap();
	}

	//let one:BigInt = ToBigInt::to_bigint(&1).unwrap();
	let mut max:BigInt = Zero::zero();
	let mut i = 0; // trailing pointer, j will be leading pointer
	let mut next:u32;
	let mut prod:BigInt = One::one();

	for j in 0..999 { //
		next = nums[j];
		// next number is zero, reset running product to 1, bump trailing pointer to the next number
		if next == 0 {
			prod = One::one();
			i = j + 1;
		}
		else {
			// if window is maxed out, divide by number at trailing pointer
			if j - i > 12 {
				prod = prod / nums[i].to_bigint().unwrap();
				i += 1;
			}
			// multiply by next number
			prod = prod * next.to_bigint().unwrap();

			if prod > max {
				max = prod.clone(); // hmm
			}
		}
	}
	max
}

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc
fn pe9() -> u64 {
	let res:u64 = 0;
	for x in 1..499 {
		for y in x..499 {
			for z in y..499 {
				if (x*x) + (y*y) == (z*z) && x + y + z == 1000 {
					return x * y * z;
				}
			}
		}
	}
	res
}


//Find the sum of all the primes below two million
// slow but whatever
fn pe10() -> u64 {
	let mut primes = vec![2,3,5,7,11];
	let mut sum:u64 = primes.iter().sum();
	let mut next: u64;
	loop {
		add_next_prime(&mut primes);
		next = *primes.last().unwrap();
		if next < 2000000 {
			sum = sum + next;
		}
		else {
			break;
		}
	}
	sum
}
