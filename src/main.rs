
struct Item {
    what: String,
    present: bool,
}

struct Hands {
    left: Item,
    right: Item,
}

fn main() {
    let hands: Hands = Hands {
        left: Item {
            what: "an apple".to_owned(),
            present: true,
        },
        right: Item {
            what: "an banana".to_owned(),
            present: true,
        },
    };
    
    if hands.left.present {
        println!("Left hand is holding {}", hands.left.what);
    } else {
        println!("Left hand is not holding!");
    }

    if hands.right.present {
        println!("Right hand is holding {}", hands.right.what);
    } else {
        println!("Right hand is not holding!");
    }
}
