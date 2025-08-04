use json_ifc::ControlledAccess;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    Privileged,
    Unprivileged,
}

#[derive(ControlledAccess, Clone, Serialize)]
struct Testing {
    #[access(Role::Privileged, Role::Unprivileged)]
    name: String,
    #[access(Role::Privileged)]
    age: u32,
}

const FILE_PATH: &'static str = "testing_data.json";

fn create_file() {
    let my_testing = Testing {
        name: "Tommy".to_string(),
        age: 32,
    };
    let as_string = serde_json::to_string(&my_testing).unwrap();
    // Write to file
    std::fs::write(FILE_PATH, &as_string).unwrap();
}

fn read_file() -> String {
    std::fs::read_to_string(FILE_PATH).unwrap()
}

fn main() {
    // create ex file
    create_file();

    // read contents of ex file
    let file_contents = read_file();

    // parse into a privileged accessor - can access both fields
    let privileged: TestingPrivilegedAccessor = serde_json::from_str(&file_contents).unwrap();
    println!("{}", privileged.name());
    println!("{}", privileged.age());

    // parse into a UNprivileged accessor - can access only name()
    let unprivileged: TestingUnprivilegedAccessor = serde_json::from_str(&file_contents).unwrap();
    println!("{}", unprivileged.name());
    // EX: The types will prevent this since `age` is sensitive
    // println!("{}", unprivileged.age());
}
