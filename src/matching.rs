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
    rewriting_if_let();
}

//a match that runs code when the value matches one pattern and then ignores all others
fn rewriting_if_let(){
    let my_number = Some(1u8);
    match my_number {
        Some(x) => println!("inside match first arm {}",x),
        _=>(),
    }

    if let Some(x) = my_number {
        println!("inside match first arm but with sugar {}",x)
    } else {
        println!("inside match other arms")
    }
}