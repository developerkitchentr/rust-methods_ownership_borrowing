use model::Hands;
mod model {


    pub trait Displayable {
        fn display(&self) -> String;
    }

    pub enum Fruit {
        Apple,
        Banana,
        Kiwi
    }

    impl Displayable for Fruit {
        fn display(&self) -> String {
            match self {
                Fruit::Apple => "an Apple".to_owned(),
                Fruit::Banana => "a Banana".to_owned(),
                _ => "a Kiwi".to_owned()
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
                left: Option::Some(Fruit::Apple),
                right: Option::Some(Fruit::Banana),
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

    pub fn report_item<T: Displayable>(item: &Option<T>, which: &str) {
        match item {
            Option::Some(what) => {
                println!("{} hand is holding {}", which, what.display());
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
