/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{LookupMap, Vector};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct FN{
    pub id:u32,
    pub hash_head: String,
    pub hash_body: String,
    pub uri: String
}
#[near_bindgen]
impl FN{
    pub fn clone(&self)->FN{
        FN{
            id: self.id.clone(), 
            hash_head: self.hash_head.clone(), 
            hash_body: self.hash_body.clone(),
            uri: self.uri.clone()
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct News{
    news: Vec<FN>,
    counter: u32,
}

#[near_bindgen]
impl News{
    pub fn add(&mut self, hash_head: String, hash_body:String, uri: String){
        self.counter+=1;
        self.news.push(FN{
            id: self.counter, 
            hash_head, 
            hash_body, 
            uri
        });
    }

    pub fn display_all(&self){
        for i in self.news.iter(){
            println!("{}. {}", &i.id, &i.uri);
        }
    }

    pub fn display_by_index(&self, index: u32){
        if (index as usize) < self.news.len(){
            println!("Element {:#?} on index {}", self.news[index as usize].clone(), index);
        }
    }

    pub fn get_by_index(&self, index: u32)-> FN{
        self.news[index as usize].clone()
    }

    pub fn clone(&self) -> Vec<FN>{
        let mut news_clone: Vec<FN>= Vec::new();
        for i in self.news.iter(){
            news_clone.push(i.clone());
        }
        news_clone
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test(){
        let p = FN{id: 0, hash_head: "jsd4vfj".to_string(), hash_body: "hrsh465".to_string(), uri: "uri".to_string()};
        let mut news: News = News{ news: Vec::new(), counter: 0};
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());

        println!("---------");
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        news.display_all();
        println!("---------");
        news.display_by_index(1);

        let a = news.get_by_index(1);
        println!("A:  {:#?} ", a);
        assert!(news.news.len()>0);
    }
}