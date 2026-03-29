fn main() {
    let mut mana = 100;

    // This closure BORROWS mana mutably
    let mut cast_spell = || {
        mana -= 10;
        println!("Cast! Mana left: {}", mana);
    };

    cast_spell();
    let _cast_spell = move || println!("Mana was {}", mana);
}