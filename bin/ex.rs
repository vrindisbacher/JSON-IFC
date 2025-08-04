use json_ifc::ControlledAccess;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    Privileged,
    Unprivileged,
}

#[derive(ControlledAccess, Clone)]
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
    let privileged = TestingPrivilegedAccessor::new(my_testing.clone());
    println!("{}", privileged.name());


    let unprivileged = TestingUnprivilegedAccessor::new(my_testing);
    println!("{}", unprivileged.name());
    // EX: The types will prevent this since `age` is sensitive
    // println!("{}", original.age());
}
