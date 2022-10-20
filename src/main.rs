use x0::bank::{deposit, withdraw, Account};
use std::{
    thread,
    sync::{Arc, Mutex}
};

fn main() {
    let user_account = Arc::new(Mutex::new(Account {
        wallet: 1000
    }));

    let one = Arc::clone(&user_account);
    let thread_deposit = thread::spawn(move || {
        for _ in 0..=10 {
            deposit(&one);
        }
    });
    
    let two = Arc::clone(&user_account);
    let thread_withdraw = thread::spawn(move || {
        for _ in 0..=10 {
            withdraw(&two);
        }
    });

    thread_deposit.join().unwrap();
    
    thread_withdraw.join().unwrap();
    println!("Waller = {:?}", user_account);
}
