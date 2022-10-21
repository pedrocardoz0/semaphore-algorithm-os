use x0::bank::{deposit, withdraw, Account, Smaphore};
use std::{
    thread,
    time,
    sync::{Arc, Mutex}
};

fn main() {
    let ten_millis = time::Duration::from_millis(1000);
    
    let user_account = Arc::new(Mutex::new(Account {
        wallet: 1000
    }));

    let mut smaphore = Smaphore { sign: true };

    let one = Arc::clone(&user_account);
    let thread_deposit = thread::spawn(move || {
        if smaphore.sign {
            println!("\n\nSemafaro: {:?}\n\n", smaphore.sign);

            smaphore.sign = false;
            for _ in 0..=10 {
                deposit(&one);
            }
            println!("\n\nSemafaro: {:?}\n\n", smaphore.sign);
            smaphore.sign = true;
        }
    });
    
    let two = Arc::clone(&user_account);
    let thread_withdraw = thread::spawn(move || {
        thread::sleep(ten_millis);
        if smaphore.sign {
            println!("\n\nSemafaro: {:?}\n\n", smaphore.sign);

            smaphore.sign = false;
            for _ in 0..=10 {
                withdraw(&two);
            }
            println!("\n\nSemafaro: {:?}\n\n", smaphore.sign);
            smaphore.sign = true; 
        }
    });

    thread_deposit.join().unwrap();
    
    thread_withdraw.join().unwrap();
    println!("Wallet = {:?}", user_account);
}
