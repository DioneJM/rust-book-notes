
#[derive(Debug)]
enum UsState {
    Alaska, 
    Alabama
}

#[derive(Debug)]
enum Coin {
    Penny,
    Dime, 
    Nickel,
    Quarter(UsState), 
}

fn coins() {
    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("coins - State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn coins_with_if_let() {
    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("coins with if let - State quarter from {:?}!", state)
    } else {
        count += 1;
    }
}

fn main() {
    coins();
    coins_with_if_let();
}
