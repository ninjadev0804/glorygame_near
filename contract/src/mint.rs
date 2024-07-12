use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        // token_id: TokenId,
        // metadata: TokenMetadata,
        // receiver_id: AccountId,
        //we add an optional parameter for perpetual royalties
        // perpetual_royalties: Option<HashMap<AccountId, u32>>,
    ) {
        const OGSALE_TIME: u64 = 1663851600000; // 13th June 2022 04:00PM UTC
        const APPRENTICESALE_TIME: u64 = 1663853400000; // 13th June 2022 05:00PM UTC
        const WLSALE_TIME: u64 = 1663855200000; // 13th June 2022 05:00PM UTC
        const PUBSALE_TIME: u64 = 1663857000000; // 13th June 2022 05:00PM UTC

        const OWNERSALE_LIMIT: u128 = 538; // 13th June 2022 05:00PM UTC
        const OGSALE_LIMIT: u128 = 3; // 13th June 2022 05:00PM UTC
        const APPRENTICESALE_LIMIT: u128 = 2; // 13th June 2022 05:00PM UTC
        const WLSALE_LIMIT: u128 = 1; // 13th June 2022 05:00PM UTC
        const PUBSALE_LIMIT: u128 = 3; // 13th June 2022 05:00PM UTC

        let my_token_id = self.token_metadata_by_id.len() + 1;

        assert!(my_token_id < 539, "Minting ended");

        let account_id = env::predecessor_account_id();

        let amount = env::attached_deposit();

        if amount == 0 {
            let end = self.contributor_0.len();
            let mut exist = false;
            for i in 0..end {
                if account_id == self.contributor_0.get(i).unwrap(){
                    if self.nft_supply_for_owner(account_id.clone()).0 >= OWNERSALE_LIMIT {
                        self.contributor_0.swap_remove(i);
                        exist = false;
                        break;
                    }
                    exist = true;
                    break;
                }
            }
            if exist == false {
                if (env::block_timestamp() / 1000000) < OGSALE_TIME {
                    env::panic(b"You can't mint before NFT sale");           
                } else if (env::block_timestamp() / 1000000) > OGSALE_TIME && (env::block_timestamp() / 1000000) < APPRENTICESALE_TIME {
                    let end = self.contributor_4.len();
                    let mut exist = false;
                    for i in 0..end {
                        if account_id == self.contributor_4.get(i).unwrap(){
                            if self.nft_supply_for_owner(account_id.clone()).0 >= OGSALE_LIMIT {
                                self.contributor_4.swap_remove(i);
                                exist = false;
                                break;
                            }
                            exist = true;
                            break;
                        }
                    }
                    if exist == false {
                        env::panic(b"You are not OG user or You can't mint the more than 3");
                    }
                } else if (env::block_timestamp() / 1000000) > APPRENTICESALE_TIME && (env::block_timestamp() / 1000000) < WLSALE_TIME {
                    let end = self.contributor_7.len();
                    let mut exist = false;
                    for i in 0..end {
                        if account_id == self.contributor_7.get(i).unwrap(){
                            if self.nft_supply_for_owner(account_id.clone()).0 >= APPRENTICESALE_LIMIT {
                                self.contributor_7.swap_remove(i);
                                exist = false;
                                break;
                            }
                            exist = true;
                            break;
                        }
                    }
                    if exist == false {
                        env::panic(b"You are not Apprentice user or You can't mint the more than 2");
                    }
                } else if (env::block_timestamp() / 1000000) > WLSALE_TIME && (env::block_timestamp() / 1000000) < PUBSALE_TIME {
                    let end = self.contributor_10.len();
                    let mut exist = false;
                    for i in 0..end {
                        if account_id == self.contributor_10.get(i).unwrap(){
                            if self.nft_supply_for_owner(account_id.clone()).0 >= WLSALE_LIMIT {
                                self.contributor_10.swap_remove(i);
                                exist = false;
                                break;
                            }
                            exist = true;
                            break;
                        }
                    }
                    let end1 = self.contributor_14.len();
                    for i in 0..end1 {
                        if account_id == self.contributor_14.get(i).unwrap(){
                            if self.nft_supply_for_owner(account_id.clone()).0 >= WLSALE_LIMIT {
                                self.contributor_14.swap_remove(i);
                                exist = false;
                                break;
                            }
                            exist = true;
                            break;
                        }
                    }
                    if exist == false {
                        env::panic(b"You are not Whitelist user or You can't mint the more than 1");
                    }
                } else if (env::block_timestamp() / 1000000) > PUBSALE_TIME {
                    let mut exist = false;
                    if self.nft_supply_for_owner(account_id.clone()).0 >= PUBSALE_LIMIT {
                        exist = false
                    } else {
                        exist = true;
                    }
                    if exist == false {
                        env::panic(b"You can't mint the more than 3");
                    }
                }
                // env::panic(b"You can't mint the more than 2 in free");
            }
        } else if amount == 5000000000000000000000000 {
            if (env::block_timestamp() / 1000000) < OGSALE_TIME {
                env::panic(b"You can't mint before NFT sale");           
            } else if (env::block_timestamp() / 1000000) > OGSALE_TIME && (env::block_timestamp() / 1000000) < APPRENTICESALE_TIME {
                let end = self.contributor_4.len();
                let mut exist = false;
                for i in 0..end {
                    if account_id == self.contributor_4.get(i).unwrap(){
                        if self.nft_supply_for_owner(account_id.clone()).0 >= OGSALE_LIMIT {
                            self.contributor_4.swap_remove(i);
                            exist = false;
                            break;
                        }
                        exist = true;
                        break;
                    }
                }
                if exist == false {
                    env::panic(b"You are not OG user or You can't mint the more than 3");
                }
            } else if (env::block_timestamp() / 1000000) > APPRENTICESALE_TIME && (env::block_timestamp() / 1000000) < WLSALE_TIME {
                let end = self.contributor_7.len();
                let mut exist = false;
                for i in 0..end {
                    if account_id == self.contributor_7.get(i).unwrap(){
                        if self.nft_supply_for_owner(account_id.clone()).0 >= APPRENTICESALE_LIMIT {
                            self.contributor_7.swap_remove(i);
                            exist = false;
                            break;
                        }
                        exist = true;
                        break;
                    }
                }
                if exist == false {
                    env::panic(b"You are not Apprentice user or You can't mint the more than 2");
                }
            } else if (env::block_timestamp() / 1000000) > WLSALE_TIME && (env::block_timestamp() / 1000000) < PUBSALE_TIME {
                let end = self.contributor_10.len();
                let mut exist = false;
                for i in 0..end {
                    if account_id == self.contributor_10.get(i).unwrap(){
                        if self.nft_supply_for_owner(account_id.clone()).0 >= WLSALE_LIMIT {
                            self.contributor_10.swap_remove(i);
                            exist = false;
                            break;
                        }
                        exist = true;
                        break;
                    }
                }
                let end1 = self.contributor_14.len();
                for i in 0..end1 {
                    if account_id == self.contributor_14.get(i).unwrap(){
                        if self.nft_supply_for_owner(account_id.clone()).0 >= WLSALE_LIMIT {
                            self.contributor_14.swap_remove(i);
                            exist = false;
                            break;
                        }
                        exist = true;
                        break;
                    }
                }
                if exist == false {
                    env::panic(b"You are not Whitelist user or You can't mint the more than 1");
                }
            } else if (env::block_timestamp() / 1000000) > PUBSALE_TIME {
                let mut exist = false;
                if self.nft_supply_for_owner(account_id.clone()).0 >= PUBSALE_LIMIT {
                    exist = false
                } else {
                    exist = true;
                }
                if exist == false {
                    env::panic(b"You can't mint the more than 3");
                }
            }
        } else if amount != 5000000000000000000000000 {
            env::panic(b"Require correct amount of Near attached");
        }

        let mut token_id = my_token_id.to_string();

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();
        royalty.insert("glorygames.near".to_string().try_into().unwrap(), 1000);

        if amount > 0 {
            Promise::new("bd.paras.near".to_string().try_into().unwrap()).transfer(87500000000000000000000);
        }

        // if perpetual royalties were passed into the function: 
        // if let Some(perpetual_royalties) = perpetual_royalties {
        //     //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
        //     assert!(perpetual_royalties.len() < 7, "Cannot add more than 6 perpetual royalty amounts");

        //     //iterate through the perpetual royalties and insert the account and amount in the royalty map
        //     for (account, amount) in perpetual_royalties {
        //         royalty.insert(account, amount);
        //     }
        // }

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: account_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &TokenMetadata{
            title: Some("The Glory Game #".to_owned() + token_id.as_ref()),
            description: Some("A 538 collection introducing the Glory Games lore and character consisting of PETs, Avatars, in-game NFT consumables and egg shards.".to_owned()),
            media: Some("https://glorygames.mypinata.cloud/ipfs/QmXaSNi8eVJR4nqNASufuHCyw3Y3qENBHitpGNdbwr4D49/".to_owned() + token_id.clone().as_ref() + ".mp4"),
            media_hash: None,
            copies: None,
            issued_at: Some(env::block_timestamp() / 1000000),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: Some("https://glorygames.mypinata.cloud/ipfs/QmeFdQa1wtrAq4mVrhuKhZDLLnSF9FT1M6rX6ib91NWt7j/".to_owned() + token_id.clone().as_ref() + ".json"),
            reference_hash: None
        });

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        // let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        // refund_deposit(required_storage_in_bytes);
    }
    pub fn check_token(&self, id:TokenId)->bool{
        self.tokens_by_id.contains_key(&id)
    }

    pub fn get_total_supply(&self) -> u64 {
        self.token_metadata_by_id.len()
    }
}