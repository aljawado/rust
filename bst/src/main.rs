use std::io;

struct Employee {
    name: String,
    age: u8,
    id: i64,
    title: String
}

impl Employee {
    fn display(&self) {
        println!("name={}, age={}, id={}, title={}", self.name, self.age, self.id, self.title);
    }
}

fn main() {
    let mut employee = Employee {
        name: String::from("Joe Doe"),
        age: 50,
        id: 123456789,
        title: String::from("SWE")
    };

    println!("Current employee is:");
    employee.display();

    println!("Enter new employee name: ");
    io::stdin().read_line(&mut employee.name)
        .expect("Error reading name");

    println!("Employee has been changed to:");
    employee.display();

    let mut test = String::from("This is a test");
    println!("{}", test);

    test = String::from("rrrrrrrrr");
    println!("{}", test);

}
