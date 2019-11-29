use rebell::Hunspell;

fn main() {
    let mut hun = Hunspell::new(String::from("/Users/Thomas/Library/Spelling/fr-toutesvariantes.aff"),
    String::from("/Users/Thomas/Library/Spelling/fr-toutesvariantes.dic")).unwrap();

    println!("{:?}", hun.suggest(String::from("Coukou")));
}
