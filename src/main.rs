
struct Item {
    what: String,
    present: bool,
}

struct Hands {
    left: Item,
    right: Item,
}

#[allow(clippy::manual_swap)]
fn main() {

    // air ile yaptığımız swap işleminin işe yaraması için hands
    // değişkenini mutable yapıyoruz.
    let mut hands: Hands = Hands {
        left: Item {
            what: "an apple".to_owned(),
            present: true,
        },
        right: Item {
            what: "an banana".to_owned(),
            present: true,
        },
    };
    
    report(&hands); 
    juggle(&mut hands);
    report(&hands);
}

fn juggle(hands: &mut Hands){
    println!("Let's juggle");
    std::mem::swap(&mut hands.left, &mut hands.right)
    //let air = hands.left;
    //hands.left = hands.right;
    //hands.right = air;
}

fn report(hands: &Hands)  {
    // Refactoring yapıyoruz ve tekrarlayan bölümleri 
    // ayırıyoruz
    report_item(&hands.left, "Left");
    report_item(&hands.right, "Right");
}

fn report_item(item: &Item, which: &str) {
    if item.present {
        println!("{} hand is holding {}", which, item.what);
    } else {
        println!("{} hand is not holding!", which);
    }
}
