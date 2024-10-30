use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let parsed_data = Grammar::parse(Rule::field, "-273.5")?;
    println!("{:?}", parsed_data);

    Ok(())
}
