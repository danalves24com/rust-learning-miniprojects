 struct Dog {
     name  : String,
     age   : u16,
     breed : Breed
}

impl Dog {
    fn verbose_boi(&self) -> String {
        let verbose_type : &str = match self.breed {
            Breed::Goodboi    => "good boi",
            Breed::Puffyboi   => "puffy boi",
            Breed::Torpedoboi => "torpedo boi"
        };
        format!("is a {}", String::from(verbose_type))
    }
}

#[derive(Debug)]
enum Breed {
    Goodboi,
    Puffyboi,
    Torpedoboi
}

fn main() {

    let dog = Dog {
        name  : String::from("Sani"),
        age   : 3,
        breed : Breed::Goodboi
    };


    println!("{} {}",&dog.name,&dog.verbose_boi());
}
