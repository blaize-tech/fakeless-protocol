use std::collections::HashSet;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[derive(Serialize, Deserialize, BorshDeserialize, Debug, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct News
{
    pub id: u32,
    pub hash_head: String,
    pub hash_body: String,
    pub uri: String,
    pub like: u64,
    pub dislike: u64,
    pub voted: HashSet<String>, 
    pub creator: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug,  Clone)]
pub struct NewsStorage
{
    news: Vec<News>,
}


#[near_bindgen]
impl NewsStorage
{
    pub fn add(&mut self, hash_head: String, hash_body:String, uri: String)
    {
        self.news.push(News
            {
                id: (self.news.len() as u32)+1, 
                hash_head, 
                hash_body, 
                uri,
                like: 0,
                dislike: 0,
                voted: HashSet::new(),
                creator: env::signer_account_id(),
            }
        );
    }

    pub fn get_all(&self)-> Vec<News>
    {
        self.news.clone()
    }

    pub fn get_by_index(&self, index: usize)-> News
    {
        self.news[index].clone()
    }

    pub fn upvote(&mut self, index: usize) 
    {
        let log_message = format!("This account is {}", env::signer_account_id());
            env::log(log_message.as_bytes());
        assert!(index < self.news.len());
        if !self.news[index].voted.contains(&env::signer_account_id())
        {
            self.news[index].like = self.news[index].like.saturating_add(1);
            self.news[index].voted.insert(env::signer_account_id().clone()); //// signer_account_id ? ////

            let log_message = format!("This account is {}", env::signer_account_id());
            env::log(log_message.as_bytes());
        }
        else
        {

            let log_message = format!("This account {} has already voted", env::signer_account_id());
            env::log(log_message.as_bytes());
        }
        
    }

    pub fn downvote(&mut self, index: usize) 
    {
        assert!(index < self.news.len());
        if !self.news[index].voted.contains(&env::signer_account_id())
        {
            self.news[index].dislike = self.news[index].dislike.saturating_add(1);
            self.news[index].voted.insert(env::signer_account_id().clone());
        }
        else
        {
            let log_message = format!("This account {} has already voted", env::signer_account_id());
            env::log(log_message.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn add_test()
    {
        let mut t_news: NewsStorage = NewsStorage::default();
        t_news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        assert_eq!(t_news.news.len(), 1);
    }

     #[test]
    fn downvote_test()
    {
        let mut t_news: NewsStorage = NewsStorage::default();
        t_news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        t_news.downvote(0);
        assert_eq!(t_news.news[0].dislike, 1);
    }

    #[test]
    fn upvote_test()
    {
        let mut t_news: NewsStorage = NewsStorage::default();
        t_news.add("sht54".to_string(),"jsbvkd465".to_string(), "uri".to_string());
        t_news.upvote(0);
        assert_eq!(t_news.news[0].like, 1);
    }

    #[test]
    fn get_by_index_test()
    {
        let mut news_storage = NewsStorage::default();
        news_storage.add(String::from(""), String::from(""), String::from(""));
        assert_eq!(news_storage.news[0].id, news_storage.get_by_index(0).id);
    }

    #[test]
    fn get_all_test()
    {
        let mut news_storage = NewsStorage::default();
        news_storage.add(String::from(""), String::from(""), String::from(""));
        assert_eq!(news_storage.news.len(), news_storage.get_all().len());
    }

}