  struct Student {
        name: String,
        major: String,
    }
    impl Student {
        fn new(name: String, major: String) -> Self {
            Self {name, major }
        }
        fn get_major(&self) -> &String {
            &self.major
        }
        fn set_major(&mut self, new_major:String) {
            self.major = new_major;
        }
    }

fn main() {
    let mut student = Student::new("Mayeli".to_string(), "Computer Science".to_string());
    //println!("Student name: {}", student.get_name());
    println!("Student Major: {}", student.get_major());
    student.set_major("Computer Science".to_string());
    println!("Major {}", student.get_major());
}