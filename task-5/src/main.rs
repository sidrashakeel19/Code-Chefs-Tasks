// Topics: Conditional, Loops, math

// Given an input find the list of all the prime factors and check if the given number 
// is an Armstrong number.

// Our previous task was on Prime numbers and factors so I don't need to explain the concepts.
// Armstrong number is a number which is equal to the sum of its digits raised to the power 
// of total digits. For eg. 9 = 9 ^ 1(armstrong number), 27 = 2^2 + 7 ^ 2(not an armstrong number), 371 = 3 ^ 3, 7 ^ 3, 1 ^3 = 371 (armstrong number) and so on..


fn main() {
  let mut num: u32 = 5;
  let temp = num;
  let mut sum = 0;
  while num > 0 {
    let r = num%10;
    sum = sum + (r*r*r);
    num = num/10;
  }
  if temp==sum {
      println!("This is an armstrong number");
  }
  else {
      println!("This is not an armstrong number");
  }
}