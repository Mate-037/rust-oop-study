struct Student {
    name: String,
    age: u32,
    grade: f32,
}
impl Student {
    //method to display student information
    fn display(&self) {
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade);
    }
    //method to increment age
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy Birthday, {}! You are now {} years old.", self.name, self.age);
    }
    fn has_passed(&self) -> bool {
        self.grade >= 60.0
    }
}
fn main() {
    let mut student1 = Student {
        name: String::from("Alice"),
        age: 20,
        grade: 85.5,
    };
    let mut student2 = Student {
        name: String::from("Bob"),
        age: 22,
        grade: 55.0,
    };

    student1.display();
    student2.display();

    student1.have_birthday();
    student2.have_birthday();

    println!("Has {} passed? {}", student1.name, student1.has_passed());
    println!("Has {} passed? {}", student2.name, student2.has_passed());
}