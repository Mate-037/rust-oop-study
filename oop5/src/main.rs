struct Student {
    name: String,
    age: u32,
    grade: f32,
}
struct Teacher {
    name: String,
    subject: String,
}
trait Person {
    fn greet(&self);
    fn info(&self) -> String;
}
/*
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
    */

    
impl Person for Student {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
    fn info(&self) -> String {
        format!("{} is {} years old and has a grade of {}.", self.name, self.age, self.grade)
    }
}
impl Person for Teacher {
    fn greet(&self) {
        println!("Hello, my name is {} and I teach {}!", self.name, self.subject);
    }
    fn info(&self) -> String {
        format!("{} teaches {}.", self.name, self.subject)
    }
}
fn main() {
    let s1 = Student {
        name: String::from("Alice"),
        age: 20,
        grade: 85.5,
    };
    let s2 = Student {
        name: String::from("Bob"),
        age: 22,
        grade: 55.0,
    };
    let t1 = Teacher {
        name: String::from("Mr. Smith"),
        subject: String::from("Math"),
    };
    let t2 = Teacher {
        name: String::from("Ms. Johnson"),
        subject: String::from("History"),
    };
    let people: Vec<Box<dyn Person >> = vec![
        Box::new(s1),
        Box::new(s2),
        Box::new(t1),
        Box::new(t2),
    ];
    println!("----Greetings:----");
    for person in people {
        person.greet();
        println!("{}", person.info());
        println!("--Greeting Completed--");

        println!("{} says goodbye!", person.info());
    }
}