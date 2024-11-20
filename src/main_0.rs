trait Description {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Description for Person {
    fn describe(&self) -> String {
        format!("Nome: {}, Idade: {}", self.name, self.age)
    }
}

enum Job {
    Employed(String),
    Unemployed,
}

impl Description for Job {
    fn describe(&self) -> String {
        match self {
            Job::Employed(position) => format!("Cargo: {}", position),
            Job::Unemployed => String::from("Desempregado"),
        }
    }
}
fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 30,
    };

    let job1 = Job::Employed(String::from("Engenheiro de ML"));
    let job2 = Job::Unemployed;

    println!("{} {}", person.describe(), job1.describe());
    println!("Job1 {}", job1.describe());
    println!("Job2 {}", job2.describe());
}
