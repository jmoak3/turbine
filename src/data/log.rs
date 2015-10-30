extern crate rand;
extern crate crypto;
extern crate rustc_serialize;
extern crate postgres;
extern crate chrono;

// use std::os;
// use std::sync;

//  This is only a 1-to-1 transaction.
//  TODO: Implement multi-sigs
pub struct log {
    id      :   String,  //  id hash of transaction
    nonce   :   i64,     //  cryptographic nonce
    origin  :   [u8; 20],   //  origin account address
    target  :   [u8; 20],   //  target account address
    fuel    :   i64,    //  fuel of log (positive or negative fuel)
    sig     :   [u8; 30],   //  Modify with Electrum style signatures
}

//  TODO: Implement transaction receipts. Sprint 4
// struct log_receipt;

fn new_log (block_id: [u8; 30], log_id: [u8; 30], origin_address: [u8; 20], target_address: [u8; 20]){
    let new_log = 
         log{ id : new_id
              nonce   :   0,   
              origin  :   origin_address,
              target  :   target_address,
              fuel    :   0,
              sig     :   [0; 30]};
    store_log(new_log);
}

fn find_log (id : String){
    //Does this return an object, or fill out references passed as param?
    //
    conn.execute("SELECT * FROM log WHERE id == $1",
                  &id,
                  &[&log.id, 
                    &log.nonce,
                    &log.origin,
                    &log.target)
                    &log.fuel,
                    &log.sig]),
            .unwrap();
}

fn store_log (l : log){
    conn.execute("INSERT INTO log \
                  id      :   String
                  nonce   :   bigint,   
                  origin  :   bytea,
                  target  :   bytea,
                  fuel    :   bigint,
                  sig     :   bytea,
                  VALUES ($1, $2, $3, $4, $5, $6)",
                  &[&log.id, 
                    &log.nonce,
                    &log.origin,
                    &log.target)
                    &log.fuel,
                    &log.sig]),
            .unwrap();
}

fn remove_log (id : String){
    conn.execute("DELETE FROM log \
                  WHERE id == $1",
                  &id)
            .unwrap();
}

pub fn setup_block_table(){
    conn.execute("CREATE TABLE log (
                  id      :   String,  //  id hash of transaction
                  nonce   :   i64,     //  cryptographic nonce
                  origin  :   [u8; 20],   //  origin account address
                  target  :   [u8; 20],   //  target account address
                  fuel    :   i64,    //  fuel of log (positive or negative fuel)
                  sig     :   [u8; 30],   //  Modify with Electrum style signatures
                  )", &[]).unwrap();
}