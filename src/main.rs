use hoi4save::{Hoi4File, Encoding, EnvTokens};

mod models;

fn main() {
    let data = 
        std::fs::read("C:\\Users\\Admin\\Documents\\Paradox Interactive\\Hearts of Iron IV\\save games\\Ironman   144564564654564.hoi4").unwrap();
    let file = hoi4save::Hoi4File::from_slice(&data).unwrap();

    let parsed_file = file.parse().unwrap();

    let save: crate::models::Hoi4Save = parsed_file.deserializer(&EnvTokens).deserialize().unwrap();
    
    for i in &save.mods {
        println!("{}", i);
    }
}
