import { WalletConnection } from "near-api-js";
import React, { useEffect, useState } from "react";
import { walletConnect } from "../utils";

interface FormValue {
  uri: string,
  header: string,
  body: string,
}

const initialState = {
  uri: "",
  header: "",
  body: "",
};

function AddNews() {

    const [wallet, setWallet] = useState({} as WalletConnection);
    const [contract, setContract] = useState({} as any);

    useEffect(() => {
      async function fetchMyAPI() {
        let { wallet, contract } = await walletConnect();
        setWallet(wallet);
        setContract(contract);
      }

      fetchMyAPI()
    }, [])

    const useForm = (callback: any, initialState: FormValue) => {
      const [values, setValues] = useState(initialState);
    
      const onInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
          setValues({ ...values, [event.target.name]: event.target.value });
      };
      const onTextAreaChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setValues({ ...values, [event.target.name]: event.target.value });
    };
      const onSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        await callback(); 
      };
    
      return {
        onInputChange,
        onSubmit,
        onTextAreaChange,
        setValues,
        values,
      };
    }

    // getting the event handlers from our custom hook
    const { onInputChange, onSubmit, onTextAreaChange, setValues, values } = useForm(
        loginUserCallback,
        initialState
    );

    async function digestMessage(message: string) {
      const msgUint8 = new TextEncoder().encode(message);                           // encode as (utf-8) Uint8Array
      const hashBuffer = await crypto.subtle.digest('SHA-256', msgUint8);           // hash the message
      const hashArray = Array.from(new Uint8Array(hashBuffer));                     // convert buffer to byte array
      const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join(''); // convert bytes to hex string
      return hashHex;
    }

    // a submit function that will execute upon form submission
    async function loginUserCallback() {
        // const hash_head = await digestMessage(values.header);
        // const hash_body = await digestMessage(values.body);
        const result = {
          uri: values.uri,
          hash_head: values.header,
          hash_body: values.body,
        }

        // Call contract method
        await contract.add(result);
        window.location.href = "/news";

        setValues(initialState);
        
    }

    return (
        <div className="formWrapper">
          <form onSubmit={onSubmit} className="formclass">
            <div><input name='uri' id='uri' type='uri' placeholder='uri' value={values.uri} pattern='(https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|www\.[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.[^\s]{2,}|https?:\/\/(?:www\.|(?!www))[a-zA-Z0-9]+\.[^\s]{2,}|www\.[a-zA-Z0-9]+\.[^\s]{2,})' onChange={onInputChange} required /></div>
            <div><input name='header' id='header' type='input' placeholder='header' onChange={onInputChange} value={values.header} required /></div>
            <div><textarea name='body' id='body' placeholder='body' value={values.body} onChange={onTextAreaChange} required /></div>
            <button type='submit'>Submit</button>
          </form>
        </div>
    );
}

export default AddNews;