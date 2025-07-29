//Number Analyzer



fn is_even(n: i32) -> bool
{
    if n % 2 == 0
    {return true;}

    else 
    {return false;} 
}



fn main() {
//create an arrray of 10 int nums 
let nums: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    
    
for i in nums
{
    if is_even(i)
    {println!("even");}

    if i % 3 == 0 && i % 5 == 0
    {println!("FizzBuzz");}

    if i % 3 == 0
   {println!("Fizz");}

    if i % 5 == 0
    {println!("Buzz");}
   
  
}//end of for loop 

let mut i = 0;
let mut sum = 0;

while i < nums.len()
{
    sum += nums[i];
    i += 1;
}
println!("The sum is: {}",sum);

let mut l = nums[0];
let mut i = 1;

while i < nums.len()
{
    if nums[i] > l
    {l = nums[i];}

    i+= 1;
}

 println!("The largest number is: {}", l);
}//end of main
