// structs are effectively like classes
// struct names are always capitalized

// left of colon: list of fields(data) that the struct will wrap up
// right of colon: data type and data contained in it

// Vectors are variable-length arrays that can grow and shrink in size
// Arrays in Rust have fixed lengths

#[derive(Debug)] // Defines attributes for the Deck struct. 'Derive' attribute specifies which
                 // traits to automatically implement for struct. Traits are sets of functions.
                 // think of this as automatically extending the struct with builtins
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // variables are called bindings
    // declare a new binding variable, create an instance of a struct, and create an empty vector for cards
    // alternative way of writing vec![] is Vec::new()
    let deck = Deck { cards: vec![] };

    // using a debug formatter to print out this data
    println!("Here is your deck: {:?}", deck);
}
