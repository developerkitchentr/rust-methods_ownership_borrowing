use model::{Hands };
mod model {
    pub struct Item {
        pub what: String,
        pub present: bool,
    }
    pub struct Hands {
        pub left: Item,
        pub right: Item,
    }
    impl Hands {
        pub fn new()-> Self {
            Hands {
                    left: Item {
                        what: "an apple".to_owned(),
                        present: true,
                    },
                    right: Item {
                        what: "an banana".to_owned(),
                        present: true,
                    },
            }
        }
        pub fn juggle(&mut self){
            println!("Let's juggle");
            std::mem::swap(&mut self.left, &mut self.right)
        }

        pub fn report(&self)  {
            self.left.report_item("Left");
            self.right.report_item("Right");
        }
    }

    impl Item {
        pub fn report_item(&self, which: &str) {
            if self.present {
                println!("{} hand is holding {}", which, self.what);
            } else {
                println!("{} hand is not holding!", which);
            }
        }
    }
}
fn main() {
    let mut hands: Hands = Hands::new();    
    hands.report(); 
    hands.juggle();
    hands.report();
}
