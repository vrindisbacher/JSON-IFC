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

fn main() {
    let my_testing = Testing {
        name: "Tommy".to_string(),
        age: 32,
    };
    let as_string = serde_json::to_string(&my_testing).unwrap();
    // Write to file
    std::fs::write("testing_data.json", &as_string).unwrap();

    // Read back from file
    let file_contents = std::fs::read_to_string("testing_data.json").unwrap();
    println!("File contents: {}", file_contents);

    let privileged: TestingPrivilegedAccessor = serde_json::from_str(&file_contents).unwrap();
    println!("{}", privileged.name());
    println!("{}", privileged.age());

    let unprivileged: TestingUnprivilegedAccessor = serde_json::from_str(&file_contents).unwrap();
    println!("{}", unprivileged.name());
    // EX: The types will prevent this since `age` is sensitive
    // println!("{}", unprivileged.age());
}
