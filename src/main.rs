use std::{
    sync::{Arc, Mutex},
    thread,
};
use x0::bank::{deposit, withdraw, Account, Smaphore};

fn main() {
    let user_account = Arc::new(Mutex::new(Account { wallet: 1000 }));

    let mut smaphore = Smaphore { sign: true };

    let one = Arc::clone(&user_account);
    let thread_deposit = thread::spawn(move || {
        if smaphore.sign {
            smaphore.sign = false;
            for _ in 0..=10 {
                deposit(&one);
            }
            smaphore.sign = true;
        }
    });

    let two = Arc::clone(&user_account);
    let thread_withdraw = thread::spawn(move || {
        if smaphore.sign {
            smaphore.sign = false;
            for _ in 0..=10 {
                withdraw(&two);
            }
            smaphore.sign = true;
        }
    });

    thread_deposit.join().unwrap();

    thread_withdraw.join().unwrap();
    println!("Waller = {:?}", user_account);
}
