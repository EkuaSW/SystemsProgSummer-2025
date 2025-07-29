 
 fn word_enhancer( mut word:String) -> String
    {
        word.push_str("RGV");
        word

    }

fn main() {
    println!("Hello, Dorcas!");

   let  mut s = "UT".to_string(); // Allocates memory on the heap
   word_enhancer(s);
    
   println!("{}", s);
}

