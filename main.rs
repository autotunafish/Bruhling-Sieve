//John R Bruhling
//02-05-23 21:36
//autotunafish@yahoo.com
//
//The Bruhling Sieve
//
//This article and executable program describe and demonstrate the conjecture
//for what has been dubbed The Bruhling Sieve with a fast prime number 
//counting algorithm simply called 'fastcounter'. The premise of the conjecture 
//is as follows: 
//
//
//
//Any prime number applied a MOD function with the value 30 will result in one  
//of 8 values between 1 and 30. Those values are 1 | 7 | 11 | 13 | 17 | 19 | 23 | 29 
//Any prime number (>5) that does not yield one of the 8 values listed 
//above from a MOD 30 function applied cannot possibly be prime. 
//
//The omitted values 2, 3, and 5 are technically prime but only appear once as 
//MOD values and for the coresponding integers 2, 3, and 5 while the others are 
//repeatedly called. These three values are totally 
//exclusive to themselves while all of the other MOD values are shared amongst 
//all the other primes and it was with this in mind that 24 was 
//decided upon as the number of MOD values given in the description despite the 
//minor technicalities.
//
//
//
//This notion is derived from a turtle graphics project which
//resulted in an image called the 96 Paths of Primality and can be viewed here
//https://archive.org/details/ulamlogspiral
//The image shows an archimedian spiral of length 360 per loop wher the actual
//pixel length of each single line grows relatively to mimic a relatively 
//increasing velocity (first loop 1-360, second 361-720, third 
//721-1080..repeat). Along the spiral are plotted the primes as they occured 
//and very clearly can be seen, perhaps not a discernable order of occurances
//but certainly a type of periodicity in the locations relative to the number 
//line which in that case was base 360 and the 'line' as a circle (I like to 
//think of it all as a cylinder shape). As stated the spiral mirrors itself and
//it suffices to use a base 30 (MOD 30) to get the correct result. 
//This algorithm eliminates having to evaluate 4 of the 12 odd numbers in any 
//linear set of len 30 and reduces the need of all prime evaluations in 
//the counting program by nearly half.
//
//


//For benchmarking
use std::time::{Instant};

//Starts the program :p
fn main() {

	//Starts the benchmark
   let now = Instant::now();
   
	//This holds the running total of primes found in the range
	let mut cont = 0;
	
	//This sets the range. 0..=611953 in particular holds 50000 primes
	//0..=15485863 holds 1,000,000 primes
	let height = 611953;
	
	//Iterate through the range. 
     for _i in 0..=height {
     
     //This quickly filters all even numbers except 2 itself
	if _i != 2 {	
		if _i % 2 == 0 {
			continue
		}
	}
	     
	//This quickly filters all numbers div 5 except 5 itself
	if _i != 5 {	
		if _i % 5 == 0 {
			continue
		}
	}
	//This gets the MOD 30 of the odd number.
	//If it is not one of the values below then it is not prime
     let md = _i % 30;
			
			//2,3,5 are just for the integers "2,3 and 5" and only appear
			//for those integers. These and any other numbers in the match can 
			//verified as being effective by simply removing it and replacing 
			//it in the second test-arm. This will print the designated values 
			//w/o affecting the total prime count.
			//The enire match md{} can be removed to run the prime check 
			//further below on every odd number to cross-check the output as 
			//well.
			//There is a println near the bottom to only print the 2,3 and 5 
			//MOD values when they occur to verify that they only occur once.
			
			//All primes of any size applied a MOD 30 function will yield one
			//of these values below. Else not possibly prime.
			match md {
				1 | 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 => (),
				
				//	test-arm is preset for removing the value 13 above.
				//Anytime the _i MOD 30 = 13 it will print.
				//13 => { println!("{}..{}", &_i, &md); }
				
				//cannot possibly be prime.
				_ => continue

			}
     //holds prime check
     let mut result = true;
     
     //min check value
     let count = _i.clone() as f64;
     
			//numbers 2 and up could be prime
			if count >= 2.0 {
			
				//Find square root of count
				let sqrt = count.sqrt();
				
				//truncate, round up
				let mut trsqrt = sqrt.clone().trunc() + 1.0; 
				
				//Check all the modulus of the real values from the 
				//rounded up square root down to 2.
				while trsqrt >= 2.0 {
				
					//If any == 0.0 then it is composite, else prime.
					if count % trsqrt == 0.0 {
						result = false;
						}
						
					//down to 2
					trsqrt -= 1.0;
					}
					
				// 2 is prime
				if count == 2.0 {
					result = true;
				}
			}
		// 1 is not prime
		if count <= 1.0 { 
			result = false;
		}
		
		//If prime then add to the sum of primes found
		if result == true {
		
			//This increments the total found
			cont = cont + 1;
			
			//This prints all the primes, its MOD and cont.
			//println!("{}..{}..{}", &_i, &md, &cont);
			
			//This prints whenever 2,3 or 5 are the MOD
			//match md {
			//	2 | 3| 5 => println!("{}..{}", &_i, &md),
			//	_ => ()
			//}
		} 
	}
	
	//This prints the range, number of primes found and the benchmark
	println!("In 0...{}", &height);
	println!("total primes: {}", &cont);
	println!("total millis: {}", now.elapsed().as_millis());
}

//Note
//Performance was increased a non-trivial amount by increasing the mod value of "md = _i % 30" to 90, 180,
//360, etc and applying a larger match statement and effectively performing less computations overall. 
//The following works for mod 360
// 1 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 | 37 | 41 | 43 | 47 | 49 | 53 | 59 | 61 | 67 | 71 | 73 | 77 | 79 | 83 | 89 |
//91 | 97 | 101 | 103 | 107 | 109 | 113 | 119 | 121 | 127 | 131 | 133 | 137 | 139 | 143 | 149 | 151 | 157 | 161 | 163 | 167 | 169 | 173 | 179 | 				 
//181 | 187 | 191 | 193 | 197 | 199 | 203 | 209 | 211 | 217 | 221 | 223 | 227 | 229 | 233 | 239 | 241 | 247 | 251 | 253 | 257 | 259 | 263 | 269 |
// 271 | 277 | 281 | 283 | 287 | 289 | 293 | 299 | 301 | 307 | 311 | 313 | 317 | 319 | 323 | 329 | 331 | 337 | 341 | 343 | 347 | 349 | 353 | 359 |

