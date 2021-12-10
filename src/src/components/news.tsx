import { WalletConnection } from 'near-api-js';
import React, { useEffect, useState } from 'react';
import { viewMethodOnContract, walletConnect } from '../utils';

export interface NewsItem {
  id: number
  hash_head: string;
  hash_body: string;
  uri: string;
  like: number;
  dislike: number
}

interface PropsList {
  list: NewsItem[];
}

interface PropsListItem {
  item: NewsItem;
}


function News() {

  const [news, setNews] = useState([]);
  const [wallet, setWallet] = useState({} as WalletConnection);
  const [contractApi, setContract] = useState({} as any);

  useEffect(() => {
    async function fetchMyAPI() {
      let response = await viewMethodOnContract("get_all");
      setNews(response);
    }

    fetchMyAPI()
  }, [])

  useEffect(() => {
    async function fetchMyAPI() {
      let { wallet, contract } = await walletConnect();
      setContract(contract);
      setWallet(wallet);
    }

    fetchMyAPI()
  }, [])

  function Vote(type: string, newsId: number) {
    alert(`${type}  ${newsId}`);
    // if(type === 'like') {
    //   contractApi.vote_like(newsId);
    // } else if (type === 'dislike') {
    //   contractApi.vote_dislike(newsId);
    // }
  }

  const List: React.FC<PropsList> = ({list}) => (
    <ul className="listWrap" >
      {list.map(item => (
        <ListItem key={item.id} item={item} />
      ))}
    </ul>
  );

  const ListItem: React.FC<PropsListItem> = ({ item }) => (
    <li key={item.id}>
      <div>{item.hash_head}</div>
      <div>{item.hash_body}</div>
      <div>{item.uri}</div>
      <div>{item.like} <img src="https://toppng.com/uploads/preview/reddit-clipart-icon-reddit-upvote-transparent-11562895696nryk8bvsps.png" alt="like" onClick={(e) => Vote('like', item.id)} /></div>
      <div>{item.dislike} <img src="https://mpng.subpng.com/20180414/ckq/kisspng-reddit-github-imgur-emoji-down-arrow-5ad2494934f857.803642991523730761217.jpg" alt="dislike" onClick={(e) => Vote('dislike', item.id)} /></div>
    </li>
  );
  

  return (
    <div className="newsPage">
      <List list={news} />
    </div>
  );
}

export default News;