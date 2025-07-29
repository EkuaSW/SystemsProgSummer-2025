
fn check_guess(guess: i32, secret: i32) -> i32
{
    if guess == secret
    {return 0;}

    if guess > secret
    {return 1;}
    
    else {return -1;}
}




fn main() {

let mut secret = 40;
let mut guess = 25;
let mut count = 0;

while guess != secret
{
    if check_guess(guess,secret) == 0
    {count += 1;
    println!("YAAAAAY!!!! You guesses correctly!!!");
    break;}

    else if check_guess(guess,secret) == 1
    {count += 1;
     guess -= 1;
     println!("Your guess is too high")}

    else if check_guess(guess,secret) == -1
    {count += 1;
     guess += 1;
     println!("Your guess is too low")}

   

}

println!("The game has ended. you guessed {} times !", count);




   
}//end of main 
