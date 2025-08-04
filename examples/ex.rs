use json_access_control::{AccessRoles, ControlledAccess, access_guard};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, AccessRoles)]
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

#[access_guard(Role::Privileged)]
struct PrivilegedGuard;

// #[check_controlled_access]
// fn test_check_controlled_access(_: PrivilegedGuard) -> TestingPrivilegedAccessor {
//     let file_contents: String = read_file();
//     let privileged: TestingPrivilegedAccessor = serde_json::from_str(&file_contents).unwrap();
//     return privileged;
// }

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
    let file_contents: String = read_file();

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
