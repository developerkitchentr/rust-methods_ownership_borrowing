use model::Hands;
mod model {


    pub enum Fruit {
        Apple,
        Banana
    }

    impl Fruit {
        fn display(&self) -> String {
            if let Fruit::Apple = self {
                "an Apple".to_owned()
            } else {
                "a Banana".to_owned()
            }
        }
    }

    pub enum Item {
        Something(Fruit),
        Nothing
    }
    pub struct Hands {
        pub left: Item,
        pub right: Item,
    }
    impl Hands {
        pub fn new()-> Self {
            Hands {
                left: Item::Something(Fruit::Apple),
                right: Item::Something(Fruit::Banana),
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
                println!("{} hand is holding {}", which, what.display());
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
