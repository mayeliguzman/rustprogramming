
struct Car {
    seats: u8,
    model: String,
}

impl Car {
    fn new(s:u8,m:String) -> Car { 
        Self {
            seats: s,
            model: m,
        }
    }

    fn get_model(&self) -> &String {
        return &self.model
    }

    fn set_model(&mut self, new_model: String){
        self.model = new_model
    }


    struct Student {
        name: String,
        major: String,
    }
    impl Student {
        fn new(name: String, major: String) -> Self {
            Self {name, major }
        }
        fn get_major(&self) -> &String {
            %self.major
        }
        fn set_major(&mut self, new_major:String) {
            self.major = new_major;
        }
    }

}

fn main(){
    let mut my_car = Car::new(4, "Tacoma".to_string());
    println!("Number of seats {}", my_car.seats);
    println!("Number of seats {}", my_car.get_model());
    my_car.set_model("Corolla");

    println!("Number of seats {}", my_car.get_model());
}