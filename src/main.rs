use typeshare_example::Command;

fn main() {
    let command = Command::Detail(42);
    let simple_struct = typeshare_example::SimpleStruct {
        id: 42,
        name: "Hello".to_string(),
    };
    let j = serde_json::to_string_pretty(&command).unwrap();

    println!("{}", serde_json::to_string_pretty(&simple_struct).unwrap());
}
