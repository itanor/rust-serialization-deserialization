use serde::{Serialize, Deserialize};
use serde_json::{Result, Value, json};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    let serialized = serde_json::to_string(&address)?;
    println!("{}", serialized);

    Ok(())
}

fn main() -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }
        "#;

    let v: Value = serde_json::from_str(data)?;
    println!("call {} at the number {}", v["name"], v["phones"][0]);

    let p: Person = serde_json::from_str(data)?;
    println!("call {} at the number {}", p.name, p.phones[1]);
    println!("{:?}", p);

    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("name {}", john["name"]);
    println!("{}", john.to_string());

    let full_name = "Mary Conors";
    let age_last_year = 42;
    fn random_phone() -> String {
        String::from("990909090")
    }

    let mary = json!({
        "name": full_name,
        "age": age_last_year + 1,
        "phones": [
            format!("+55 {}", random_phone())
        ]
    });
    println!("{}", mary.to_string());

    let _ = print_an_address();

    Ok(())
}
