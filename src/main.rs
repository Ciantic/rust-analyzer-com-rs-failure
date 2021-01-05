use com::{interfaces, interfaces::iunknown::IUnknown};

interfaces! {
    #[uuid("EFF8970E-C50F-45E0-9284-291CE5A6F771")]
    pub unsafe interface IAnimal: IUnknown {
        pub fn Happiness(&self) -> usize;
    }
}

fn main() {
    println!("Hello, world!"); 
}