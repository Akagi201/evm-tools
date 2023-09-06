use convert_case::{Case, Casing};
use ethers::contract::Abigen;

fn bindgen(contract_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let abi_path = format!("./abi/{}.json", contract_name);
    let bindings = Abigen::new(contract_name, abi_path)?.generate()?;

    bindings.write_to_file(format!(
        "./src/bindings/{}.rs",
        contract_name.from_case(Case::Camel).to_case(Case::Snake)
    ))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./abi/Counter.json");
    println!("cargo:rerun-if-changed=./src/bindings/*.rs");

    bindgen("Counter")?;

    Ok(())
}
