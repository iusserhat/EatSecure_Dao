use ic_cdk::*;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::vec::Vec;



#[derive(CandidType, Deserialize,Clone)]
pub struct Product{
    id: Text;
    name: Text;
    production_date: Text;
    expiration_date: Text;
    image: Vec<u8>;

}


#[derive(CandidType, Deserialize,Clone)]
pub struct Info{
    desc: Text;
    production_date: Text;
    expiration_date: Text;
    product_accuracy: Bool;
}

type ProductInfo =Vec<Info>;

struct ProductRegistry{
    product_list: Arc<Mutex<HashMap<String, Product>>>, 
    info_list: Arc<Mutex<HashMap<String, ProductInfo>>>, 
    owner_list: Arc<Mutex<HashMap<String, Pet>>>,
}



impl  ProductRegistry{

    pub fn new() -> Self {

        pet_list= Arc::new(Mutex::new(HashMap::new())),
        info_list: Arc::new(Mutex::new(HashMap::new())),

}

