
import React, { useState, useEffect } from 'react';
import { BrowserRouter, Route, Switch } from "react-router-dom";
import { keyStores } from 'near-api-js';
import * as buffer from "buffer"
import WalletProvider from './contexts/wallet';
import Landing from './Components/Landing';

function App() {
  const [keyStore, setKeyStore] = useState();
  useEffect(() => {
    window.Buffer = buffer.Buffer
    const keyStore = new keyStores.BrowserLocalStorageKeyStore();
    setKeyStore(keyStore);
  }, [])

  return (
    <div style={{ height: '100vh' }} className="App">
      <BrowserRouter>
        <WalletProvider keyStore={keyStore}>
          <Switch>
            <Route exact path='/'>
              <Landing />
            </Route>
          </Switch>
        </WalletProvider>
      </BrowserRouter>
    </div>
  );
}

export default App;
