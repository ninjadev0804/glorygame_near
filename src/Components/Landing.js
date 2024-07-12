import React from "react";
import { useContext, useEffect, useState, useRef } from "react";
import { parseNearAmount } from "near-api-js/lib/utils/format";
import { NotificationContainer, NotificationManager } from 'react-notifications';
import Countdown from 'react-countdown';
import 'react-notifications/lib/notifications.css';
import { WalletContext, NFT_CONTRACT_ID, MAX_GAS } from "../contexts/wallet";
import "./style.css";
function Landing() {
  const [isloading, setLoading] = React.useState(false);

  const { wallet, signIn, signOut } = useContext(WalletContext);
  const freelimit = 434;
  const totalSupply = 538;
  const [nftSupply, setNftSupply] = useState(0);

  let ogtime = 1663851600000;
  let apprenticetime = 1663853400000;
  let wltime = 1663855200000;
  let pubtime = 1663857000000;

  // let localtime = Date.now()
  let utctime = new Date(new Date().toUTCString()).getTime();

  const notstart = () => {
    NotificationManager.warning("NFT Sale is not started yet")
  }

  const success = () => {
    NotificationManager.success("NFT minted!")
  }

  const connectwallet = () => {
    NotificationManager.warning("Please connect wallet!")
  }

  const onWallet = async () => {
    signIn();
  }

  function refreshPage() {
    window.location.reload(false);
  }

  const WalletDisconnect = async () => {
    signOut();
    refreshPage();
  }

  useEffect(() => {
    const intervalId = setInterval(() => {
      (async () => {
        if (wallet && wallet.isSignedIn()) {
          const total_supply = await wallet.account().viewFunction(NFT_CONTRACT_ID, "get_total_supply");
          setNftSupply(total_supply);
        }
      })();
    }, 2000)
    return () => clearInterval(intervalId);
  }, [wallet])

  const onMint = async () => {
    if (!wallet.isSignedIn()) {
      connectwallet();
      return;
    }

    const contributor_0 = await wallet.account().viewFunction(NFT_CONTRACT_ID, "get_contributor_0");

    let mint_price = "0";
    if (nftSupply && nftSupply > freelimit) {
      mint_price = "5";
      if (contributor_0.includes(wallet.getAccountId())) {
        mint_price = "0";
      }
    }

    try {
      setLoading(true);
      await wallet.account().functionCall(
        NFT_CONTRACT_ID,
        "nft_mint",
        {
        },
        MAX_GAS,
        parseNearAmount(mint_price)
      )
      success();
    } catch (err) {
      NotificationManager.error(JSON.parse(err.message).kind.ExecutionError.replace("Smart contract panicked: ", ""))
    }
    setLoading(false);
  }

  const videoEl = useRef(null);
  const attemptPlay = () => {
    videoEl &&
      videoEl.current &&
      videoEl.current.play().catch((error) => {
        console.error("Error attempting to play", error);
      });
  };

  useEffect(() => {
    attemptPlay();
  }, []);

  const Completionist1 = () => <span>OG Sale Started</span>;
  const Completionist2 = () => <span>OG Sale Ended and Apprentice Sale Started</span>;
  const Completionist3 = () => <span>Apprentice Sale Ended and Whitelist Sale Started</span>;
  const Completionist4 = () => <span>Whitelist Sale Ended and Public Sale Started</span>;

  const renderer1 = ({ days, hours, minutes, seconds, completed }) => {
    if (completed) {
      // Render a completed state
      return <Completionist1 />;
    } else {
      // Render a countdown
      return <span>{days} days {hours} hours {minutes} minutes {seconds} seconds</span>;
    }
  };

  const renderer2 = ({ days, hours, minutes, seconds, completed }) => {
    if (completed) {
      // Render a completed state
      return <Completionist2 />;
    } else {
      // Render a countdown
      return <span>{days} days {hours} hours {minutes} minutes {seconds} seconds</span>;
    }
  };

  const renderer3 = ({ days, hours, minutes, seconds, completed }) => {
    if (completed) {
      // Render a completed state
      return <Completionist3 />;
    } else {
      // Render a countdown
      return <span>{days} days {hours} hours {minutes} minutes {seconds} seconds</span>;
    }
  };

  const renderer4 = ({ days, hours, minutes, seconds, completed }) => {
    if (completed) {
      // Render a completed state
      return <Completionist4 />;
    } else {
      // Render a countdown
      return <span>{days} days {hours} hours {minutes} minutes {seconds} seconds</span>;
    }
  };

  return (
    <div className="landing">
      <NotificationContainer />

      <div className="header">
        <div className="container">
          <div className="row">
            <div className="col-sm-2 text-center justify-content-center align-self-center">
              <a className="header_logo" href="#">
                <img src="/images/items/Glory_Games_Logo.png" alt="Logo" />
              </a>
            </div>
            <div className="col-sm-6 text-center text-sm-start pt-3 pt-sm-0 justify-content-center align-self-center">
              <div className="game_header_info">
                <h2 className="game_head_title">
                  Gaia Archives NFT Mint - Trading Cards Collection{" "}
                </h2>
                <div className="game_head_desc event_start_title">
                  <span>Minting Live 538 Supply</span>
                </div>
                <div className="game_head_desc">
                  <span className="timer_title">OG/Alpha:</span>
                  <div className="countdown" id="og_countdown">
                    <Countdown
                      date={ogtime}
                      renderer={renderer1}
                    />
                  </div>
                </div>
                <div className="game_head_desc">
                  <span className="timer_title">Apprentice:</span>
                  <div className="countdown" id="apprentice_countdown">
                    <Countdown
                      date={apprenticetime}
                      renderer={renderer2}
                    />
                  </div>
                </div>
                <div className="game_head_desc">
                  <span className="timer_title">Whitelist:</span>
                  <div className="countdown" id="whitelist_countdown">
                    <Countdown
                      date={wltime}
                      renderer={renderer3}
                    />
                  </div>
                </div>
                <div className="game_head_desc">
                  <span className="timer_title">Public:</span>
                  <div className="countdown" id="whitelist_countdown">
                    <Countdown
                      date={pubtime}
                      renderer={renderer4}
                    />
                  </div>
                </div>
              </div>
            </div>
            <div className="col-sm-2 offset-sm-2 offset-md-0 pt-3 pt-sm-0 col-md-4 justify-content-center text-sm-end text-center align-self-center">
              {wallet && wallet.isSignedIn() ?
                <div
                  className="btn wallet_btn text-black bg-white text-uppercase"
                  onClick={WalletDisconnect}
                >
                  {wallet.getAccountId().slice(0, 5)}...{wallet.getAccountId().slice(-5)}
                </div>
                :
                <div
                  className="btn wallet_btn text-black bg-white text-uppercase"
                  onClick={onWallet}
                >
                  Connect Wallet
                </div>
              }
            </div>
          </div>
        </div>
      </div>
      <div className="container pt-5">
        <p className="nft-title">Collect and Win In-game NFTs</p>
        <div className="row pt-5">
          {/* <!--<img src="Genesis_Pets.png" className="pet_img"/>--> */}
          {/* <video autoplay muted loop className="pet_video">
            <source src="/images/items/adverse.mp4" type="video/mp4" /> */}
          <video
            style={{ margin: "0 auto" }}
            playsInline
            loop
            muted
            controls
            alt="All the devices"
            src="/images/items/adverse.mp4"
            ref={videoEl}
          />
          {/* </video> */}
        </div>
        <div className="row pt-5">
          <p className="nft-title">Mint Gaia Archives Cards</p>
          <div className="col-sm-4 left_part">
            <video id="nft_pet_video" width="450px">
              <source src="/images/items/final.mp4" type="video/mp4" />
            </video>
          </div>
          <div className="col-sm-8 right_part">
            <div className="min_form bg-black text-white p-3">
              {/* <form method="POST"> */}
              <div className="field_row row border-bottom p-2">
                <div className="field_label col-6">Price</div>
                <div className="field_desc_box col-6 text-end">
                  <span className="price_value_box">
                    <span className="price_value" name="petonly_price">
                      {nftSupply > freelimit ? '0.1' : '0'}
                    </span>
                    &nbsp;N
                  </span>
                </div>
              </div>
              <div className="field_row row border-bottom p-2">
                <div className="field_label col-6">Total</div>
                <div className="field_desc_box col-6 text-end">
                  <div className="total_price" name="petonly_total_price">
                    {totalSupply}
                  </div>
                </div>
              </div>
              <div className="field_row row border-bottom p-2">
                <div className="field_label col-6">Remaining</div>
                <div className="field_desc_box col-6 text-end">
                  <div
                    className="total_price_remaining"
                    name="petonly_total_price_remaining"
                  >
                    {totalSupply - nftSupply}
                  </div>
                </div>
              </div>

              <div className="submit_btn text-center pt-3"
              >
                {
                  isloading ?
                    <div
                      className="btn wallet_btn text-center text-black bg-white text-uppercase"
                    >
                      Minting ...
                    </div>
                    :
                    <div
                      className="btn wallet_btn text-center text-black bg-white text-uppercase"
                      onClick={onMint}
                    >
                      mint now
                    </div>
                }
              </div>
              {/* </form> */}
            </div>
          </div>
        </div>
      </div>
      <div className="py-5 mt-5 lucky-section">
        <div className="container">
          <div className="d-flex flex-row justify-content-center align-items-center">
            <i className="fa fa-star"></i>
            <i className="fa fa-star ps-2"></i>
            <h1 className="star_lucky mx-3 mb-0 text-uppercase">Feeling Lucky</h1>
            <i className="fa fa-star"></i>
            <i className="fa fa-star ps-2"></i>
          </div>
          <p className="text-center mt-2 mb-4 fw-bold fs-5">
            <span className="nft_desc">This pool contains pets/avatar/egg shards only</span>
            <span className="nft-supply d-block text-center text-uppercase">
              Final 104 Supply
            </span>
          </p>

          <img
            src="/images/items/archieve.png"
            className="d-block w-100"
            alt="Gaia Archives Chance Up"
          />

          {/* <div className="min-btn-section pt-4">
            <div>
              <span className="d-block fs-5 pb-1 text-center">0.015 ETH</span>
              <button className="btn-mint-blue">Mint 1</button>
            </div>
            <div>
              <span className="d-block text-center fs-5 pb-1">0.1 ETH</span>
              <button className="btn-mint-yellow">Mint 10</button>
              <span className="d-block text-center fs-5 pt-1">
                -33% Discount
              </span>
            </div>
          </div> */}
        </div>
      </div>

      <div className="container-fluid footer p-5">
        <div className="cotainer row">
          <div className="col-12 text-center footer_social">
            <a href="https://twitter.com/GloryGamesWorld">
              <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 1024 1024"><path fill="currentColor" d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM727.3 401.7c.3 4.7.3 9.6.3 14.4c0 146.8-111.8 315.9-316.1 315.9c-63 0-121.4-18.3-170.6-49.8c9 1 17.6 1.4 26.8 1.4c52 0 99.8-17.6 137.9-47.4c-48.8-1-89.8-33-103.8-77c17.1 2.5 32.5 2.5 50.1-2a111 111 0 0 1-88.9-109v-1.4c14.7 8.3 32 13.4 50.1 14.1a111.13 111.13 0 0 1-49.5-92.4c0-20.7 5.4-39.6 15.1-56a315.28 315.28 0 0 0 229 116.1C492 353.1 548.4 292 616.2 292c32 0 60.8 13.4 81.1 35c25.1-4.7 49.1-14.1 70.5-26.7c-8.3 25.7-25.7 47.4-48.8 61.1c22.4-2.4 44-8.6 64-17.3c-15.1 22.2-34 41.9-55.7 57.6z" /></svg>
            </a>
            <a href="https://www.instagram.com/glorygamesofficial">
              <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M12 9.52A2.48 2.48 0 1 0 14.48 12A2.48 2.48 0 0 0 12 9.52Zm9.93-2.45a6.53 6.53 0 0 0-.42-2.26a4 4 0 0 0-2.32-2.32a6.53 6.53 0 0 0-2.26-.42C15.64 2 15.26 2 12 2s-3.64 0-4.93.07a6.53 6.53 0 0 0-2.26.42a4 4 0 0 0-2.32 2.32a6.53 6.53 0 0 0-.42 2.26C2 8.36 2 8.74 2 12s0 3.64.07 4.93a6.86 6.86 0 0 0 .42 2.27a3.94 3.94 0 0 0 .91 1.4a3.89 3.89 0 0 0 1.41.91a6.53 6.53 0 0 0 2.26.42C8.36 22 8.74 22 12 22s3.64 0 4.93-.07a6.53 6.53 0 0 0 2.26-.42a3.89 3.89 0 0 0 1.41-.91a3.94 3.94 0 0 0 .91-1.4a6.6 6.6 0 0 0 .42-2.27C22 15.64 22 15.26 22 12s0-3.64-.07-4.93Zm-2.54 8a5.73 5.73 0 0 1-.39 1.8A3.86 3.86 0 0 1 16.87 19a5.73 5.73 0 0 1-1.81.35H8.94A5.73 5.73 0 0 1 7.13 19a3.51 3.51 0 0 1-1.31-.86A3.51 3.51 0 0 1 5 16.87a5.49 5.49 0 0 1-.34-1.81V8.94A5.49 5.49 0 0 1 5 7.13a3.51 3.51 0 0 1 .86-1.31A3.59 3.59 0 0 1 7.13 5a5.73 5.73 0 0 1 1.81-.35h6.12a5.73 5.73 0 0 1 1.81.35a3.51 3.51 0 0 1 1.31.86A3.51 3.51 0 0 1 19 7.13a5.73 5.73 0 0 1 .35 1.81V12c0 2.06.07 2.27.04 3.06Zm-1.6-7.44a2.38 2.38 0 0 0-1.41-1.41A4 4 0 0 0 15 6H9a4 4 0 0 0-1.38.26a2.38 2.38 0 0 0-1.41 1.36A4.27 4.27 0 0 0 6 9v6a4.27 4.27 0 0 0 .26 1.38a2.38 2.38 0 0 0 1.41 1.41a4.27 4.27 0 0 0 1.33.26h6a4 4 0 0 0 1.38-.26a2.38 2.38 0 0 0 1.41-1.41a4 4 0 0 0 .26-1.38V9a3.78 3.78 0 0 0-.26-1.38ZM12 15.82A3.81 3.81 0 0 1 8.19 12A3.82 3.82 0 1 1 12 15.82Zm4-6.89a.9.9 0 0 1 0-1.79a.9.9 0 0 1 0 1.79Z" /></svg>
            </a>
            <a href="https://discord.com/invite/qtvanFwrux">
              <svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 16 16"><path fill="currentColor" d="M13.545 2.907a13.227 13.227 0 0 0-3.257-1.011a.05.05 0 0 0-.052.025c-.141.25-.297.577-.406.833a12.19 12.19 0 0 0-3.658 0a8.258 8.258 0 0 0-.412-.833a.051.051 0 0 0-.052-.025c-1.125.194-2.22.534-3.257 1.011a.041.041 0 0 0-.021.018C.356 6.024-.213 9.047.066 12.032c.001.014.01.028.021.037a13.276 13.276 0 0 0 3.995 2.02a.05.05 0 0 0 .056-.019c.308-.42.582-.863.818-1.329a.05.05 0 0 0-.01-.059a.051.051 0 0 0-.018-.011a8.875 8.875 0 0 1-1.248-.595a.05.05 0 0 1-.02-.066a.051.051 0 0 1 .015-.019c.084-.063.168-.129.248-.195a.05.05 0 0 1 .051-.007c2.619 1.196 5.454 1.196 8.041 0a.052.052 0 0 1 .053.007c.08.066.164.132.248.195a.051.051 0 0 1-.004.085a8.254 8.254 0 0 1-1.249.594a.05.05 0 0 0-.03.03a.052.052 0 0 0 .003.041c.24.465.515.909.817 1.329a.05.05 0 0 0 .056.019a13.235 13.235 0 0 0 4.001-2.02a.049.049 0 0 0 .021-.037c.334-3.451-.559-6.449-2.366-9.106a.034.034 0 0 0-.02-.019Zm-8.198 7.307c-.789 0-1.438-.724-1.438-1.612c0-.889.637-1.613 1.438-1.613c.807 0 1.45.73 1.438 1.613c0 .888-.637 1.612-1.438 1.612Zm5.316 0c-.788 0-1.438-.724-1.438-1.612c0-.889.637-1.613 1.438-1.613c.807 0 1.451.73 1.438 1.613c0 .888-.631 1.612-1.438 1.612Z" /></svg>            </a>
          </div>
        </div>
        <div className="cotainer row pt-3">
          <div className="col-sm-12">
            <ul className="footer_menu text-center">
              <li>
                <a href="https://www.glorygames.world/">About Us</a>
              </li>
            </ul>
          </div>
        </div>
        <div className="cotainer row pt-3">
          <div className="footer_bottom col-sm-12 text-center">
            &copy;2022 Glory Games: Worlds All rights reserved.
          </div>
        </div>
      </div>
    </div>
  );
}

export default Landing;
