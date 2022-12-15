// This is slower than the Python, I'm sure I can optimize it but there's
// no point to doing so when I have a working alternative solution.

use rand::{thread_rng, RngCore};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate bitcoin_wallet;                                                    use bitcoin_wallet::{
    account::{
        Account,
        AccountAddressType,
        MasterAccount,
        Seed,
        Unlocker,
    },
    mnemonic::Mnemonic
}; 
                                                                                
extern crate bitcoin;                                                           
use bitcoin::Network;

fn gen_ma(passphrase : &str, prefix: &[u8]) -> MasterAccount {
   // Generate Seed from entropy with a fixed prefix
   let mut entropy: [u8; 32] = [0 as u8; 32];
   &entropy[0..prefix.len()].copy_from_slice(prefix);
   thread_rng().fill_bytes(&mut entropy[prefix.len()..]);
   let seed = Seed(entropy.to_vec());

   // Make a mainnet master account with no passphrase
   let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
   let mut ma = MasterAccount::from_seed(&seed, now, Network::Bitcoin, passphrase)
       .unwrap();

   let mut unlocker = Unlocker::new_for_master(&ma, passphrase).unwrap();

   // account number 0, sub-account 0 (which usually means receiver) BIP32 look-ahead 10
   let account = Account::new(&mut unlocker, AccountAddressType::P2PKH, 0, 0, 0).unwrap();
   ma.add_account(account);

   //assert!(ma.seed(Network::Bitcoin, passphrase).unwrap().0 == entropy);
   
   return ma;
}

fn main() {
   const PASSPHRASE: &str = "";
   const PREFIX_BYTES: &[u8; 5] = b"1Doge";
   let prefix_str: &str = str::from_utf8(PREFIX_BYTES).unwrap();
   loop {
     let mut ma = gen_ma(PASSPHRASE, PREFIX_BYTES);
     let addr = ma.get_mut((0,0)).unwrap().next_key().unwrap().address.clone();
     if addr.to_string().starts_with(prefix_str) {
         println!("{}", addr);
         let entropy = &ma.seed(Network::Bitcoin, PASSPHRASE).unwrap().0;
         println!("{:?}", entropy);
         let mnemonic = Mnemonic::new(entropy).unwrap();
         println!("{}", mnemonic.to_string());
         break;
     }
  }
}
