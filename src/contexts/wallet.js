import * as nearAPI from "near-api-js";
import React, { useCallback, useEffect, useState } from "react";

import {
  ConnectConfig,
  WalletConnection as WalletConnectionProps,
} from "near-api-js"
import { Provider } from "near-api-js/lib/providers";

const { KeyPair, keyStores, connect, WalletConnection, utils: { format: { formatNearAmount } } } = nearAPI;

export const NFT_CONTRACT_ID = "gloryfifth.testnet"
export const MAX_GAS = "300000000000000";

export const WalletContext = React.createContext({
  near: undefined,
  wallet: undefined,
  signIn: () => { },
  signOut: () => { },
  getNftMetadata: () => { },
  getCollectionList: () => { },
  getCollectionMetadata: () => { },
  getMainCollectionList: () => { },
})

// connect to NEAR
const WalletProvider = (props) => {
  const [near, setNear] = useState()
  const [wallet, setWallet] = useState()
  const [provider, setProvider] = useState()
  const [nearBalance, setNearBalance] = useState("")
  const { keyStore } = props;

  const signIn = () => {
    if (wallet)
      wallet.requestSignIn(NFT_CONTRACT_ID);
  };

  const signOut = () => {
    if (!wallet) return
    wallet.signOut()
    // location.replace("/")
  };

  const connectToNear = useCallback(async () => {
    try {
      if (keyStore) {
        // const config = {
        //   networkId: "mainnet",
        //   keyStore, // optional if not signing transactions
        //   nodeUrl: "https://rpc.mainnet.near.org",
        //   walletUrl: "https://wallet.mainnet.near.org",
        //   helperUrl: "https://helper.mainnet.near.org",
        //   // explorerUrl: "https://explorer.mainnet.near.org",
        //   headers: {}
        // };
        const config = {
            networkId: "testnet",
            keyStore, // optional if not signing transactions
            nodeUrl: "https://rpc.testnet.near.org",
            walletUrl: "https://wallet.testnet.near.org",
            helperUrl: "https://helper.testnet.near.org",
            explorerUrl: "https://explorer.testnet.near.org",
            headers: {}
          };
        const near = await connect(config);
        const provider = near.connection.provider;
        const wallet = new WalletConnection(near, null);
        setNear(near);
        setWallet(wallet);
        setProvider(provider);
        if (wallet && wallet.isSignedIn()) {
          let accountState = await wallet.account().state();
          setNearBalance(formatNearAmount(accountState.amount));
          let result = await wallet.account().getAccessKeys();
          let tokenKeyExist = false;
          // for(let i=0; i<result.length; i++){
          //   if(result[i].access_key.permission != 'FullAccess' && result[i].access_key.permission.receiver_id == NFT_CONTRACT_ID){
          //     tokenKeyExist = true;
          //     break;
          //   }
          // }
          // if(tokenKeyExist == false){
          //   console.log("Adding AccessKey to Token");
          //   const keyPair = KeyPair.fromRandom("ed25519");
          //   const publicKey = keyPair.getPublicKey().toString();
          //   await keyStore.setKey(config.networkId, publicKey, keyPair);
          //   await wallet.account().addKey(publicKey, 'kaizofighter_game_test_5.xuguangxia.near', [], '250000000000000000000000');
          // }
        } else {
          setNearBalance("0");
          console.log("==============================", nearBalance);
        }
      }
    } catch (error) {
      console.log(error, "error")
    }
  }, [keyStore])

  useEffect(() => {
    connectToNear()
  }, [keyStore])

  return (
    <WalletContext.Provider
      value={{ near, wallet, signIn, signOut }}
    >
      {props.children}
    </WalletContext.Provider>
  )
}

export default WalletProvider
