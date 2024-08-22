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

// Inherent implementation.
// Inherent implementations add functions to a struct.
// Use inherent impls to define methods and associated functions.
// Associated functions are class methods tied to the struct definition,
// whereas methods will operate on specific instances of a struct.
// Self is a reference to whatever type is mentioned in the parent implementation block.
// So impl Deck returns a Deck.
impl Deck {
    fn new() -> Self {
        // Macros are essentially functions that return the data type
        // List of suits
        // Arrays are only marginally faster than Vectors.
        // Arrays should be used to communicate when data sets are not going to change
        let suits = ["Hearts", "Spades", "Diamonds"];
        // List of values
        let values = ["Ace", "Two", "Three"];
        // Double-nested for loop

        // bindings (variables) are immutable (can't change) by default
        // You cannot change the value or reassign a binding
        let mut cards = Vec::new();

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card)
            }
        }

        // variables are called bindings
        // declare a new binding variable, create an instance of a struct, and create an empty vector for cards
        // alternative way of writing vec![] is Vec::new()
        // if the key and value ref are named the same, the expression can be simplified
        let deck = Deck { cards };
        return deck;
    }
}

fn main() {
    // using a debug formatter to print out this data
    // {:?} - just print out the variable
    // {:#?} - pretty print
    let deck = Deck::new();
    println!("Here is your deck: {:#?}", deck);
}
