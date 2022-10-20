use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Account {
    pub wallet: isize,
}

#[derive(Debug)]
pub struct Smaphore {
    pub sign: bool,
}

pub fn deposit(account: &Arc<Mutex<Account>>) {
    let mut saldo = account.lock().unwrap();
    saldo.wallet += 300;

    println!("deposit -> saldo: {:?}", saldo.wallet);
}

pub fn withdraw(account: &Arc<Mutex<Account>>) {
    let mut saldo = account.lock().unwrap();
    saldo.wallet -= 200;

    println!("withdraw -> saldo: {:?}", saldo.wallet);
}
