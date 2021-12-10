import { WalletConnection } from "near-api-js";
import { useEffect, useState } from "react";
import { walletConnect } from "../utils";
import React, { KeyboardEvent } from 'react';

function Wallet() {

  const [wallet, setWallet] = useState({} as WalletConnection);

  const useInputCallbacks = (initialState: string) => {
    let nearAccId = initialState;
    const onInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      nearAccId = event.target.value;
    };
    return {
      onInputChange,
      nearAccId,
    };
  }

  const {onInputChange, nearAccId} = useInputCallbacks('');

  const handleKeyboardEvent = (event: KeyboardEvent<HTMLInputElement>) => {
    if(event.key === 'Enter') signIn();
  };

  useEffect(() => {
    async function fetchMyAPI() {
      let { wallet } = await walletConnect();
      setWallet(wallet);
    }

    fetchMyAPI()
  }, [])


  function signIn() {
    wallet.requestSignIn(
      nearAccId, // contract requesting access
    );
  }

  function signOut() {
    wallet.signOut();
    window.location.reload();
  }

  function RenderLogic() {
    if(wallet && wallet.isSignedIn) {
        return (
          !wallet.isSignedIn() ?
          <ul>
              <div><input onChange={onInputChange} onKeyPress={handleKeyboardEvent} defaultValue={nearAccId} type="text" /></div>
              <div><button onClick={()=> signIn()}>sign in</button></div>
          </ul> :
          <ul>
            <div>Hello {wallet.getAccountId()}</div>
            <button onClick={() => signOut()}>sign out</button>
        </ul>
        )
    } else return (<div></div>)
  }

  return (
    <div className="wallet">
      <RenderLogic />
    </div>

  );
}

export default Wallet;