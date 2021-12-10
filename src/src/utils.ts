import * as _ from 'lodash';
import * as nearAPI from 'near-api-js';
import { CodeResult, QueryResponseKind } from 'near-api-js/lib/providers/provider';
import getConfig, { IConfig } from './config';

interface ObjectWithId {
  id: number
}

export function rightMergeById(arr1: ObjectWithId[], arr2: ObjectWithId[]) {
  return arr1.map(obj => {
    const index = arr2.findIndex(el => el["id"] == obj["id"]);
    return index ? arr2[index] : obj;
  });
};

export function getEnvConfig(): IConfig {
  return getConfig(process.env.NEAR_ENV || 'testnet');
}

export async function viewMethodOnContract(method: string) {
  const {nodeUrl, contractName} = getEnvConfig();
  const provider = new nearAPI.providers.JsonRpcProvider(nodeUrl);
  const rawResult: CodeResult = await provider.query(`call/${contractName}/${method}`, 'AQ4'); // Base 58 of '{}'

  return JSON.parse(rawResult.result.map((x) => String.fromCharCode(x)).join(''));
}

export async function walletConnect () {

  const { WalletConnection } = nearAPI;
  const nearConfig = getEnvConfig();

  const near =  await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    headers: {},
    ...nearConfig
  });

  const wallet = new WalletConnection(near, null);

  const account = wallet.account();

  const contract = new nearAPI.Contract(
    account, // the account object that is connecting
    nearConfig.contractName,
    {
      // name of contract you're connecting to
      viewMethods: ["get_all"], // view methods do not change state but usually return a value
      changeMethods: ["add"], // change methods modify state
    }
  );

  return {
    near,
    wallet,
    contract
  }
}


// export async function connect() {
//   const nearConfig = getEnvConfig();
//   // Connects to NEAR and provides `near`, `walletAccount` and `contract` objects in `window` scope
//   // Initializing connection to the NEAR node.
//   const near = await nearAPI.connect({
//     deps: {
//       keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
//     },
//     ...nearConfig
//   });

//   // Needed to access wallet login
//   const walletConnection = new nearAPI.WalletConnection(near);

//   // Initializing our contract APIs by contract name and configuration.
//   const contract = await new nearAPI.Contract(walletConnection.account(), nearConfig.contractName, {
//     // View methods are read-only – they don't modify the state, but usually return some value
//     viewMethods: ['get_num'],
//     // Change methods can modify the state, but you don't receive the returned value when called
//     changeMethods: ['increment', 'decrement', 'reset'],
//     // Sender is the account ID to initialize transactions.
//     // getAccountId() will return empty string if user is still unauthorized
//     sender: walletConnection.getAccountId()
//   });
// }