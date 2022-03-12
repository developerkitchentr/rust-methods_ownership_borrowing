
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

    println!("Let's juggle");

    // cargo clippy ile hatalarımızı kontrol ettiğimizde swap işleminde uyarı vermektedir
    // this looks like you are swapping `hands.left` and `hands.right` manually
    // Aşağıdaki metodu kullanmamızı öneriyor.
    // std::mem::swap(&mut hands.left, &mut hands.right)
    let air = hands.left;
    hands.left = hands.right;
    hands.right = air;  

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
