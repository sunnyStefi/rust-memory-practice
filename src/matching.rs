#[derive(Debug)]
enum State {
    Italy,
    Spain
}

enum Coin {
    Onecent,
    Oneeuro(State)
}

fn converter(coin: Coin) -> u8 {
    match coin {
        Coin::Onecent => 1,
        Coin::Oneeuro(my_state) => {
            println!("..coming from {:?}!", my_state);
            100
        }
    }
}

pub fn corresponding_cents(){
    println!("---------- Matching ----------");
    let euro = Coin::Oneeuro(State::Italy);
    println!("1 euro are {} cents", converter(euro));
}
