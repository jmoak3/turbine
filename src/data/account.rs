extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

use std::os;
use std::sync;
use self::rand::{Rng, OsRng};
use data::sidechain::sidechain;


//Make static mut Conn object?

pub struct account {
    //address     : [u8; 20], //  id hash of transaction
    // address     : &'a Vec<u8>,
    address     : Vec<u8>,
    t_nonce     : i64,      //  cryptographic nonce, represents number of logs from account
    fuel_level  : i64,
    // fuel_limit  : i64,
    // code        : [u8; 30], // TODO: Implement sprint 4
    sidechain   : Vec<u8>, //list of current minting chains
}

pub fn create_new_account(sidechain_add: &Vec<u8>) -> account{
    let new_address = gen_account_address();
    account{    address: new_address,
                t_nonce: 0 as i64,
                fuel_level: 0 as i64,
                sidechain: sidechain_add.to_vec(),}

    store_account(account);    
}

pub fn destroy_account(address: [u8; 30]){
    conn.execute("DELETE FROM account \
                  WHERE address == $1",
                  &[&address])
            .unwrap();
}

pub fn store_account(acc: account){
    conn.execute("INSERT INTO account \
                  (address, t_nonce, fuel_level, sidechain) \
                  VALUES ($1, $2, $3, $4)",
                  &[&account.address, 
                    &account.t_nonce,
                    &account.fuel_level,
                    &account.sidechain])
            .unwrap();
}

pub fn setup_account_table(){
    //not sure how to handle this junk
    conn = Connection::connect("postgres://postgres@localhost", &SslMode::None)
        .unwrap();

    conn.execute("CREATE TABLE account (
                    address        bytea,
                    t_nonce        bigint,
                    fuel_level     bigint,   
                    sidechain      bytea
                  )", &[]).unwrap();

    
}

pub fn destroy_account_table(){
    
}

pub fn gen_account_address() -> Vec<u8>{
    let address = rand::thread_rng().gen_iter::<u8>().take(20).collect::<Vec<u8>>();
    return address;

}
