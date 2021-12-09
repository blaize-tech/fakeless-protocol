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
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct FN{
    pub id:u32,
    pub hash_head: String,
    pub hash_body: String,
    pub uri: String,
    pub like: u64,
    pub dislike: u64,
}

#[near_bindgen]
impl FN{
    pub fn upvote(&mut self) {
        self.like = self.like.saturating_add(1);
    }
    pub fn downvote(&mut self) {
        self.dislike = self.dislike.saturating_add(1);
    }
    pub fn get_like(&self) -> &u64 {
        &self.like
    }
    pub fn get_dislike(&self) -> &u64 {
        &self.dislike
    }
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct News{
    news: Vec<FN>,
}


#[near_bindgen]
impl News{
    pub fn add(&mut self, hash_head: String, hash_body:String, uri: String){
        
        self.news.push(FN{
            id: (self.news.len() as u32)+1, 
            hash_head, 
            hash_body, 
            uri,
            like: 0,
            dislike: 0,
        });
    }

    pub fn display_all(&self){
        for i in self.news.iter(){
            println!("{}. {}. Likes {}. Dislikes {}.", &i.id, &i.uri, &i.like, &i.dislike);
        }
    }

    pub fn display_by_index(&self, index: u32){
        if (index as usize) < self.news.len(){
            println!("{}. {} . Likes {}. Dislikes {}.", 
            self.news[index as usize].id, 
            self.news[index as usize].uri,
            self.news[index as usize].like,
            self.news[index as usize].dislike);
        }
    }

    pub fn get_all(&self)-> Vec<FN>{
        self.vec_clone()
    }

    pub fn get_by_index(&self, index: u32)-> FN{
        self.news[index as usize].clone()
    }

    pub fn vec_clone(&self) -> Vec<FN>{
        let mut news_clone: Vec<FN>= Vec::new();
        for i in self.news.iter(){
            news_clone.push(i.clone());
        }
        news_clone
    }

    pub fn upvote(&mut self, index: u32) {
        if (index as usize) < self.news.len(){
        self.news[index as usize].like = self.news[index as usize].like.saturating_add(1);
        }
    }

    pub fn downvote(&mut self, index: u32) {
        if (index as usize) < self.news.len(){
        self.news[index as usize].dislike = self.news[index as usize].dislike.saturating_add(1);
        }
    }
    pub fn get_like(&self, index: u32) -> u64 {
        assert!((index as usize) < self.news.len());
        self.news[index as usize].like
    }
    pub fn get_dislike(&self, index: u32) -> u64 {
        assert!((index as usize) < self.news.len());
        self.news[index as usize].dislike
    }
}

impl Default for News{
    fn default()-> Self{
        News{ news: Vec::new()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test(){
        let mut news: News = News::default();
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        
        news.downvote(0);
        println!("---------");
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        news.upvote(1);
        
        news.display_all();
        println!("---------");
        news.display_by_index(1);

        let a = news.get_by_index(1);
        assert!(news.news.len()>0);
    }

    #[test]
    fn dislike_test(){
        let mut news: News = News::default();
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        
        news.downvote(0);

        println!("{}", &news.get_dislike(0));

        assert!(news.get_dislike(0) != (0 as u64));
    }

    #[test]
    fn like_test(){
        let mut news: News = News::default();
        news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        
        news.upvote(0);

        println!("{}", &news.get_like(0));

        assert!(news.get_like(0) != (0 as u64));
    }
}
