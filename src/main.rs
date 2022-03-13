use model::Hands;
mod model {



    use std::fmt::Display;

    pub trait Displayable {
        fn display(&self) -> String;
    }

    pub enum Fruit {
        Apple,
        Banana,
        Kiwi
    }

    impl Display for Fruit {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { 
            match self {
                Fruit::Apple => f.write_str("an Apple"),
                Fruit::Banana => f.write_str("a Banana"),
                _ => f.write_str("a Kiwi")
            }
        }
    }

    pub struct Hands {
        pub left: Option<Fruit>,
        pub right: Option<Fruit>,
    }
    impl Hands {
        pub fn new()-> Self {
            Hands {
                left: Some(Fruit::Apple),
                right: Some(Fruit::Banana),
            }
        }
        pub fn juggle(&mut self){
            println!("Let's juggle");
            std::mem::swap(&mut self.left, &mut self.right)
        }

        pub fn report(&self)  {
            report_item(&self.left, "Left");
            report_item(&self.right, "Right");
        }
    }

    pub fn report_item<T: Display>(item: &Option<T>, which: &str) {
        match item {
            Option::Some(what) => {
                println!("{} hand is holding {}", which, what);
            },
            _=> {
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
