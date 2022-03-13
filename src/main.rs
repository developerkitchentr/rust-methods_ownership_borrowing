use model::Hands;
mod model {
    pub enum Item {
        Something(String),
        Nothing
    }
    pub struct Hands {
        pub left: Item,
        pub right: Item,
    }
    impl Hands {
        pub fn new()-> Self {
            Hands {
                left: Item::Something("an apple".to_owned()),
                right: Item::Something("an banana".to_owned()),
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
            if let Item::Something(what) = self {
                println!("{} hand is holding {}", which, what);
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
