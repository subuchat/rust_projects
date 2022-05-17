#[derive(Debug)]
enum Taka{
    ektaka,
    dutkaa,
    panchtaka
}
#[derive(Debug)]
enum Coin{
    Dosana,
    Charana,
    Athana,
    Soloana(Taka)
}

fn value_in_paisa(coin : Coin) -> u32 {
    match coin{
        Coin::Dosana => {
            println!("Dos poisa");
            10
        }
        Coin::Charana => 25,
        Coin::Athana => 50 ,
        Coin::Soloana(taka) => {
           let kototaka = match taka {
                ektaka => 100,
                dutkaa => 200 ,
                panchtaka => 500,
            };          
            println!("You are rich {}" , kototaka);
            kototaka
        }
    }
}

fn another_func(coin : Coin) {
    match coin {
        Coin::Athana => {
            println!("TUmi athana nebe ?");
        },
        _ => println!("Sab taka tomar")

    }
}
// why without reference giving ERROR ???????????????/

fn another_func2(coin : &Coin) {
    //if let will handle only one of the element in enum , rest ignored
    if let Coin::Soloana(newvar) = coin{
       dbg!(newvar);
    }
    if let Coin::Athana = coin {
        dbg!(coin) ;       
    }
}

fn main() {
    println!("Hello, world!");
    let val = value_in_paisa(Coin::Soloana(Taka::ektaka));
    println!("value of money {}" , val);
    another_func(Coin::Soloana(Taka::dutkaa));
    another_func2(&Coin::Soloana(Taka::panchtaka));
    another_func2(&Coin::Athana);
 
}
