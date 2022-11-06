use std::env;
use std::fs;

mod table;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // 書いてて思ったけどこれ、skio_whileとtake_while使えばいいのでは？
    let position = contents
        .split("\n")
        .position(|line| line.trim().starts_with("entity") && line.trim().ends_with("{"));

    position
        .map(|pos| {
            let last = contents
                .split("\n")
                .skip(pos)
                .position(|line| line.trim().ends_with("}"));
            return last.map(|l| (pos, l));
        })
        .flatten()
        .map(|(start, end)| {
            let content = contents.split("\n").skip(start).take(end);
            let table = table::Table::new(content.collect());
            table.map(|t| t.convert_to_sql())
        });
}
