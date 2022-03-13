use model::Hands;
mod model {


    pub enum Fruit {
        Apple,
        Banana,
        Kiwi
    }

    impl Fruit {
        fn display(&self) -> String {
            match self {
                Fruit::Apple => "an Apple".to_owned(),
                Fruit::Banana => "a Banana".to_owned(),
                _ => "a Kiwi".to_owned()
            }
        }
    }

    pub enum Item<T> {
        Something(T),
        Nothing
    }
    pub struct Hands {
        pub left: Item<Fruit>,
        pub right: Item<Fruit>,
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

    impl Item<Fruit> {
        pub fn report_item(&self, which: &str) {
            match self {
                Item::Something(what) => {
                    println!("{} hand is holding {}", which, what.display());
                },
                _=> {
                    println!("{} hand is not holding!", which);
                }
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
