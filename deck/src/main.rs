use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)] // defines attributes for the Deck struct.
                 // 'derive' attribute specifies which
                 // traits to automatically implement for struct.
                 // traits are sets of functions.
                 // think of this as automatically extending the struct with builtins
struct Deck {
    // structs are effectively like classes
    // struct names are always capitalized

    // left of colon: list of fields(data) that the struct will wrap up
    // right of colon: data type and data contained in it

    // Vectors are variable-length arrays that can grow and shrink in size
    // Arrays in Rust have fixed lengths
    cards: Vec<String>,
}

impl Deck {
    // Inherent implementation.
    // Inherent implementations add functions to a struct.
    // Use inherent impls to define methods and associated functions.
    // Associated functions are class methods tied to the struct definition,
    // whereas methods will operate on specific instances of a struct.
    // Self is a reference to whatever type is mentioned in the parent implementation block.
    // So impl Deck returns a Deck.
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
        // let deck = Deck { cards };
        // return deck;
        // Implicit return, compare with two lines above
        Deck { cards }
    }

    // self also needs to be marked as mutable if you plan on messing with it
    // in this case we want to freely change our copy of the deck
    fn shuffle(&mut self) {
        // make rng mutable because the data must change over time
        let mut rng = thread_rng();
        // ... calling rands shuffle method on our cards...?
        self.cards.shuffle(&mut rng);
    }

    // broadly in rust there are signed (i) and unsigned (u) integers, and 32/64 floats
    // u/i 8-16-32-64-128, f32/64,
    // isize and usize are max size vectors/arrays -- use it to denote a size/cap into a vector
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // implicit return
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    // using a debug formatter to print out this data
    // {:?} - print out the variable
    // {:#?} - pretty print
    let mut deck = Deck::new();
    deck.shuffle();
    // probably need to add error handling
    let cards = deck.deal(3);
    println!("Here is your hand: {:#?}", cards);
    println!("Here is your deck: {:#?}", deck);
}
