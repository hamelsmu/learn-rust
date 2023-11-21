fn main() {
     let letters = vec![
         "a", "b", "c"
     ];
     let mut arr = vec![];
 
     for letter in letters {
         println!("{}", letter);
         arr.push(letter.clone());
     }
 }