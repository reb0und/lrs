#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1819,
            UsState::Alabama => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    Some(format!("{state:?}"))
}

enum Coin {
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("{max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("{state:?}");
    } else {
        count += 1;
    }
}
