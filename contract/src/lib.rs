use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};
use std::convert::TryInto;

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
    pub contributor_0: Vector<AccountId>,
    pub contributor_4: Vector<AccountId>,
    pub contributor_7: Vector<AccountId>,
    pub contributor_10: Vector<AccountId>,
    pub contributor_14: Vector<AccountId>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    Contributor0,
    Contributor4,
    Contributor7,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&&NFTContractMetadata {
                    spec: "nft-1.0.0".to_string(),
                    name: "TheGloryGames".to_string(),
                    symbol: "GLORYGAMES".to_string(),
                    icon: Some("https://glorygames.mypinata.cloud/ipfs/QmSxie5hwAq2FWsZEuuquFfKEFhZP3cMEp5k9mGGU9BVTq".to_owned()),
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                })),
            contributor_0: Vector::new(StorageKey::Contributor0.try_to_vec().unwrap()),
            contributor_4: Vector::new(StorageKey::Contributor4.try_to_vec().unwrap()),
            contributor_7: Vector::new(StorageKey::Contributor7.try_to_vec().unwrap()),
            contributor_10: Vector::new(StorageKey::Contributor7.try_to_vec().unwrap()),
            contributor_14: Vector::new(StorageKey::Contributor7.try_to_vec().unwrap()),
        };

        //return the Contract object
        this
    }

    pub fn get_contributor_0(&self) -> Vec<AccountId> {
        self.contributor_0.to_vec()
    }

    pub fn get_contributor_4(&self) -> Vec<AccountId> {
        self.contributor_4.to_vec()
    }

    pub fn get_contributor_7(&self) -> Vec<AccountId> {
        self.contributor_7.to_vec()
    }

    pub fn get_contributor_10(&self) -> Vec<AccountId> {
        self.contributor_10.to_vec()
    }

    pub fn get_contributor_14(&self) -> Vec<AccountId> {
        self.contributor_14.to_vec()
    }

    #[payable]
    pub fn init_whitelist_1(
        &mut self,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
        self.contributor_0.push(&env::predecessor_account_id().to_string().try_into().unwrap());

        self.contributor_0.push(&"glorygames.near".to_string().try_into().unwrap());
    }

    #[payable]
    pub fn init_whitelist_2(
        &mut self,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );

        self.contributor_4.push(&"sixpacklai.near".to_string().try_into().unwrap());
        self.contributor_4.push(&"midoriya_crypto.near".to_string().try_into().unwrap());
        self.contributor_4.push(&"b4befa15c053525fec307e79433eed585fcad909bcad29dab665505e7703c143".to_string().try_into().unwrap());
        self.contributor_4.push(&"tsubuyaki.near".to_string().try_into().unwrap());
        self.contributor_4.push(&"princeofgod.near".to_string().try_into().unwrap());
        self.contributor_4.push(&"d0e46f3d86baf786ed5513cc8efa11dfd45f0e22b958620d274e51b3a8701800".to_string().try_into().unwrap());
        self.contributor_4.push(&"6fb207d250e76f54d046a781b48813319904a32934000d5cee16a02b552037ff".to_string().try_into().unwrap());
    }

    #[payable]
    pub fn init_whitelist_3(
        &mut self,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );

        self.contributor_7.push(&"alexrok.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"fornears.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"697cd3fe052bb3dbca4927f0244d4d50119ceec7232e1589a425dff17fd8264a".to_string().try_into().unwrap());
        self.contributor_7.push(&"suhailasmat.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"hottieboss_91.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"levi_cryptoman.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"kkkiouuigogoi.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"poposute.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"zunleo.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"corzynya.near".to_string().try_into().unwrap());
        self.contributor_7.push(&"9753f0c6eca80f5e4b34a03913451aead41a4480b4d39b792439157881c9e908".to_string().try_into().unwrap());
    }

    #[payable]
    pub fn init_whitelist_4(
        &mut self,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
        
        self.contributor_10.push(&"sixpacklai.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"697cd3fe052bb3dbca4927f0244d4d50119ceec7232e1589a425dff17fd8264a".to_string().try_into().unwrap());
        self.contributor_10.push(&"dfce0ff6c7dc9f3e4db8ba9334dcb68c8ebed7a8ac9c757196efdc57b31b085a".to_string().try_into().unwrap());
        self.contributor_10.push(&"twhaji.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"569c927679a3520c3bb8438db5c8e02f4850d5931ed77bc280a96195a7f35df8".to_string().try_into().unwrap());
        self.contributor_10.push(&"dba1efe8037197b18c6ef1f99728784a46ee387c7f465ba7ccd5383e301cc7f2".to_string().try_into().unwrap());
        self.contributor_10.push(&"brianvuong.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"hugoyo.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"3e8ccdf4e21ded632547e64663ba14346cc527b297d82ef3ccff0607de3e6e7d".to_string().try_into().unwrap());
        self.contributor_10.push(&"fd0780ab464bdd2b87634214725a1875d97fd241c490ee60d1c8626765e92de2".to_string().try_into().unwrap());
        self.contributor_10.push(&"thangdt.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"de163085eb831d42d03e7b5f35eb47fc6886fab7c95aeb14af91e9a04952c985".to_string().try_into().unwrap());
        self.contributor_10.push(&"753952b449dfb21b3833de00de916c592c195764f120048d10c51fa91d8e7a63".to_string().try_into().unwrap());
        self.contributor_10.push(&"cbb4dfafaab8f91fbbe03a9f2f0947770a889011aca5ed3ab717ca02c3001e3f".to_string().try_into().unwrap());
        self.contributor_10.push(&"08a6c162620c7fc6eaca787c8964c82e479131e4c565f4597a8dc34afc6986d1".to_string().try_into().unwrap());
        self.contributor_10.push(&"aniket93.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"b8a4d57918b7eb8b8bd6199dae49adb0503dd40e7fc291bf8605d6d8f8be0df0".to_string().try_into().unwrap());
        self.contributor_10.push(&"topshow20.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"tsubuyaki.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"72a5efe2fcbd64908f9d2202fb44c04198901338c2eff3b3b351b6663cd46181".to_string().try_into().unwrap());
        self.contributor_10.push(&"chelin.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"37809a5eb6c050b75aedeaad65ff1a088a5895a9001d563662db8c8adf60b4f8".to_string().try_into().unwrap());
        self.contributor_10.push(&"dba337ecd15f44f75bcf37d77f9c4888453b2df2433a08c6aa68e93b9a4555ec".to_string().try_into().unwrap());
        self.contributor_10.push(&"swap6ix.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"datmap.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"94fa054f45ab979a3925a6a3583059093437f6bd55b2143a999eab0e6c620efb".to_string().try_into().unwrap());
        self.contributor_10.push(&"bshit.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"3d9f78c60e7eec6777e0573ae0ba8d83c06b1a526f9eaa5e199c3a3c0a03b19a".to_string().try_into().unwrap());
        self.contributor_10.push(&"princeofgod.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"tony2401.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"richxyz.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"7a21a162d74f87e8a3e2f861ec4a370a9b718e65c11be1d42bf276a01b220510".to_string().try_into().unwrap());
        self.contributor_10.push(&"d9a33fd959e2bab42a963efed932cc392b34bfbc58e89fe61f0b59514e1083b5".to_string().try_into().unwrap());
        self.contributor_10.push(&"9678b4bd3a07937298a322a21cf30138da2acef872cc19b4a950d493896300b9".to_string().try_into().unwrap());
        self.contributor_10.push(&"smile143.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"67bb423138a9146e4411a0a65b0a5ab050d6855fcba0bffa04746816fc399ba2".to_string().try_into().unwrap());
        self.contributor_10.push(&"fbfc01ec546143c82cbf2fd592d7dccb6e673068cf51a3b86de02a85b9d19267".to_string().try_into().unwrap());
        self.contributor_10.push(&"0bc4c42061188b05f942a63fb60324450047dac1ea218e322740168b84b69e65".to_string().try_into().unwrap());
        self.contributor_10.push(&"jhezer.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"071fd2ab53e1ef5511f7a33609c38e38c705da6694ac39696a85fd46ef1e883b".to_string().try_into().unwrap());
        self.contributor_10.push(&"dda545b309504500eb74f71b8bbedb9740256c26c39baddc4ce8529cab39a52e".to_string().try_into().unwrap());
        self.contributor_10.push(&"3b6129d9b9ccaadc8586b4015a12d681dc7a9dcb5ee364f852fb23e4bf45c7bb".to_string().try_into().unwrap());
        self.contributor_10.push(&"44d2ca7c734a9175ce19199659e620e691f10c71c4b056a0dca228ad95fe6285".to_string().try_into().unwrap());
        self.contributor_10.push(&"lilcrow.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"isho.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"antoniybogdanov.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"maxzeinly.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"25795de1570dac4498a1df9fac069c28e1d3c3f03dda1b48e32ece35071cda33".to_string().try_into().unwrap());
        self.contributor_10.push(&"datvipdh2696.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"blockchain4life.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"corzynya.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"2157e4bacb680ca40226e5bd02d6ce138ec9d32f1ad5a381f4ba5db9e4a37dda".to_string().try_into().unwrap());
        self.contributor_10.push(&"jemsarvin.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"qvexo.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"6fb207d250e76f54d046a781b48813319904a32934000d5cee16a02b552037ff".to_string().try_into().unwrap());
        self.contributor_10.push(&"gangadhar95.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"keong.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"honcauqueta.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"5bb9f8c3585b757a218cd43b0dbc46bc43e595ab836176a71754a9f71154a5ba".to_string().try_into().unwrap());
        self.contributor_10.push(&"fd9abe8c9ba33d9cb167b1bff9f50a17e970f4347caa7c40bd998ae49b184be5".to_string().try_into().unwrap());
        self.contributor_10.push(&"aa0456f48fc49c397e2454226989e0ff3faf0cfb295d38c11c94bd2c43a87594".to_string().try_into().unwrap());
        self.contributor_10.push(&"86b5d912c5638df464706e6310b05c02b0bf3b5ff34e2deac0c959b8d10c2387".to_string().try_into().unwrap());
        self.contributor_10.push(&"trungle.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"lenguyenthienminh.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"blade2runner.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"minhheo2022.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"lnta.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"lnct.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"lanato.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"truongvinh2.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"b7eaccdfb524aee9cf612775fac28157568838375985488943075d7f48db7204".to_string().try_into().unwrap());
        self.contributor_10.push(&"5ff2039372980cbd56682d625ef3427b17e6327e23fed6f8f9487b34250fca51".to_string().try_into().unwrap());
        self.contributor_10.push(&"307c73ac93a93a83de4753a43e2bbff2429f13d9ee284bad4bf8b7efba89c0f2".to_string().try_into().unwrap());
        self.contributor_10.push(&"fffc1b77ee15dc0520486e3982c1961a8dcd7d4c5149d4b16ba02d9f6193a696".to_string().try_into().unwrap());
        self.contributor_10.push(&"phoenix100.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"purple99.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"orlameday100.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"breadandbeans.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"aunttunrayo.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"yourboy.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"dfa4d566a6775a9da1803f89502a4d28f5f00b50daed9bdff151788cb3b7501c".to_string().try_into().unwrap());
        // self.contributor_10.push(&"your boy.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"vilam.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"07e7744a66c9c3b0d43d888e44c65dd5e1d3cb4601d8214ff909c571a09b93c7".to_string().try_into().unwrap());
        self.contributor_10.push(&"5945f7313967f95b2db623dcc1c2b42468a7744705fdb218a79f50ab1bfdc61f".to_string().try_into().unwrap());
        self.contributor_10.push(&"tedmeo.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"58f8819a2296ea06f7af03b5ffa81445df0d73ef78a6543db45077771e38ec86".to_string().try_into().unwrap());
        self.contributor_10.push(&"0efac563bb4f8e8101f73bdd925588ac4dfd36de845cc58cf53b09bf5b6fa7a8".to_string().try_into().unwrap());
        self.contributor_10.push(&"a32aca53f27f4e02387526e1c9f99961f6382c0160e8be0e0ada93a8f6dc9c6e".to_string().try_into().unwrap());
        self.contributor_10.push(&"5f334cbbea7d85445145fe096b0bbe968314ff46b344a8d6d3ecf17e8ff940d6".to_string().try_into().unwrap());
        self.contributor_10.push(&"9c0aa3b313dbc0e0d6074b33c2cacbdf3e46c9ea4214fb2349fbfca109532030".to_string().try_into().unwrap());
        self.contributor_10.push(&"ea8743712c46b784438b43aa9386fc641a0afa9485d61661c895e3bbd0d5161c".to_string().try_into().unwrap());
        self.contributor_10.push(&"alley205.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"2109220419d98c328ae031ecc311666b3c02507594af0ee02b2211b990667cc3".to_string().try_into().unwrap());
        self.contributor_10.push(&"9e8fb4744b688ce21c94ee37bdd8e0d94374cb0e906961b16ff9cae949361fc5".to_string().try_into().unwrap());
        self.contributor_10.push(&"def7149394c3fcd36832eb7615d67978643b12e23b608160656764cdf78f4e5e".to_string().try_into().unwrap());
        self.contributor_10.push(&"15edfa69dec5f43e48670172006c928435bc6e5fad91e673e5f758c588bfff9c".to_string().try_into().unwrap());
        self.contributor_10.push(&"2858107e22522c2d381ce18e384d42d751457d9161a24586e7b560754ec67b73".to_string().try_into().unwrap());
        self.contributor_10.push(&"leoteomiss.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"63f5a0d7eeb276d3586756237f7425bea8abe6581cadc6ddaf10a87fdf533b55".to_string().try_into().unwrap());
        self.contributor_10.push(&"yungmoney.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nikajoy.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"hipond.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"99e95531a4088a7684e8d89f5de57aa0094d7450c99e7c7e1f5416c30d63a1de".to_string().try_into().unwrap());
        self.contributor_10.push(&"c0ba096058531a05facb9c35f49ba19ae809d56328dba9fc73cf6eb40f8d50ef".to_string().try_into().unwrap());
        self.contributor_10.push(&"1dd13458afa03d0eadc54a89333daef4c78a5cc0ff1014db7dca02a411515815".to_string().try_into().unwrap());
        self.contributor_10.push(&"e06dd968cd288dadca715160759b535f56e99b63014422b931ddedd7a0bd212b".to_string().try_into().unwrap());
        self.contributor_10.push(&"f3cc61965bf79a0df57c26c9f7c179533d1d44f000c8888adc17a535ead68a97".to_string().try_into().unwrap());
        self.contributor_10.push(&"bdb3f35a76b62f4994a61859af9c5c2627a30f5d863a8c51389163fd6ec730c0".to_string().try_into().unwrap());
        self.contributor_10.push(&"5790f14c7f16bfd4007df6f9d91bedee9b638921264c7f3121ccb9a97dd02023".to_string().try_into().unwrap());
        self.contributor_10.push(&"b0f8400d69acf5a7589e7813e61db7429f8981e084f0c97f507cc0f840090e98".to_string().try_into().unwrap());
        self.contributor_10.push(&"c72f617934e9a86cd5b2af7720d6b2b983789c152a8855a166c4c1bc84b871c8".to_string().try_into().unwrap());
        self.contributor_10.push(&"a28adbec4c1cf2e9b9e124f6f9d9e08910361b498d67f3efab753bfcf2d8702d".to_string().try_into().unwrap());
        self.contributor_10.push(&"9c8a232c341163625916f3f9430cafe28971dcb47045a8bbf7750fd2c67b4ea2".to_string().try_into().unwrap());
        self.contributor_10.push(&"kaiju_crypto.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"a722c662c218d24fa995c0dbc46dcefa2005bd3204cf09f8a342b6547878f6af".to_string().try_into().unwrap());
        self.contributor_10.push(&"9263d4880ab72ed3626e8a21584ccedb4d456bf9875ea8da7d533bba7b0cf62a".to_string().try_into().unwrap());
        self.contributor_10.push(&"nponyo.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"59854df5d8c774d94a297f89f1206c6a16a482ee542775d1895f1fc390bdd417".to_string().try_into().unwrap());
        self.contributor_10.push(&"e9b2f2a35bdd7777756e96081e417d961d127e872f00f279567bd8965247c2fc".to_string().try_into().unwrap());
        self.contributor_10.push(&"e250b250719f8dc0637c141929781237355adaa0f600422f3a3fc08a5595b70e".to_string().try_into().unwrap());
        self.contributor_10.push(&"jimfemfem22.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"ruslannero.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"62b89f0f3aeef5efd478ef9ec9e3a37bb9707bd43fc9bffc52d75260cc81c68f".to_string().try_into().unwrap());
        self.contributor_10.push(&"wineyang.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"kabigbk.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"0360f3689e61c56ab18e041a18300665140e99bf41b09cf66d0a270692d6bea5".to_string().try_into().unwrap());
        self.contributor_10.push(&"3986a94395a7762da7bb10de93ae1e1feddf0986ad23e3c65885b369133b3119".to_string().try_into().unwrap());
        self.contributor_10.push(&"e7ab62b1ed713eca723277843087541f69c45d42de45062e64cd8f11c1f4d22d".to_string().try_into().unwrap());
        self.contributor_10.push(&"tailang.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"6cf399236c5a547065d1ee4e1126eba51f54ef8f5bb60f978a89edaebb59c5cc".to_string().try_into().unwrap());
        self.contributor_10.push(&"c9780c8a2d66f3d10f261d70aa9df13ef32250b524ccbf10ccac238e9d12fd1b".to_string().try_into().unwrap());
        self.contributor_10.push(&"b93949aff4a263d7d845cf3019b59084c2674adbc46ee0d095dd55266b6b61dc".to_string().try_into().unwrap());
        self.contributor_10.push(&"phantagyro.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"8e6a8c1edba5c6447efaac6d354e6a03fa5b87ce1b29c09bb3d4dffd4494e79f".to_string().try_into().unwrap());
        self.contributor_10.push(&"5298ae323c58bef1efcfde56a5197694d69abd12c311f9fb4a900bafc6524ccf".to_string().try_into().unwrap());
        self.contributor_10.push(&"chisomiloh.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"a52be72374f2f6b1d59c6161ca5eb721e41fa3d484d5adb0847cd74f1a34df5f".to_string().try_into().unwrap());
        self.contributor_10.push(&"5d1375287e8eed98ef8c06a089f408e830157df5c560efe808ee71676c22d989".to_string().try_into().unwrap());
        self.contributor_10.push(&"5becc7cd893eaeffe2b7b4c34c660552a5ee26a0a46f7e6e213c59418642a595".to_string().try_into().unwrap());
        self.contributor_10.push(&"mrtan.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"lenterasenja.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"ea87a99736fc1f0325010e2a75bea536c98d489d2ff3d586ad892dd6a09e6020".to_string().try_into().unwrap());
        self.contributor_10.push(&"05fff95199a4a3af27c6348c82f0375df0940681bc9816735d5047ed2ff24a71".to_string().try_into().unwrap());
        self.contributor_10.push(&"2644328346814b0d4dfc9465a2f8d2ab7ece5d0919521f5126d1685ac7cb4969".to_string().try_into().unwrap());
        self.contributor_10.push(&"angsazendova.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"mzie.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"81056a6e79bbbb22acc1badd43fba328abfe23a319af16faf567933bcbbd20a2".to_string().try_into().unwrap());
        self.contributor_10.push(&"nuw12.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"9330a3c2918f1d2362ed268b10482867acd9b23b4d345ac643ce3861d627b293".to_string().try_into().unwrap());
        self.contributor_10.push(&"d54170b9c5078052144adc9aa4f91c28740babc1d4f9d20504d4919a20efcc72".to_string().try_into().unwrap());
        self.contributor_10.push(&"0c4722da81005834ed76c600a77ac75e6fcb255e033c714f4c11ec4fba6f94ea".to_string().try_into().unwrap());
        self.contributor_10.push(&"bbbdaf890f94e6c154b2e2d313ed4c2e799dfe5bf4c75628beb5be027cb615a6".to_string().try_into().unwrap());
        self.contributor_10.push(&"f08da56e7fcb257102a7fbbd4f5e4243a1c9b97e3982eeb7f090eb378e19ea0d".to_string().try_into().unwrap());
        self.contributor_10.push(&"9595.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"doanhtuan2108.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"tranquang123.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"richardle061085.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"trieutuyendv.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"baolongcanada.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"2fb6ec3a6668867743c770816b3a48f5131de32ca6e676c7068c634816b99665".to_string().try_into().unwrap());
        self.contributor_10.push(&"nearholder1.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"octopusholder.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"clinomaciac28.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nftnearisgood.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"neartothemoon1.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nftisgod.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nftnearchain.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nftholder0910.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"phanzuka.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"namtheta.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyentadustin.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"6468cf60433f55024685e4524e653d7d0a1713fa51bd86c2e502bb97e90ff62a".to_string().try_into().unwrap());
        self.contributor_10.push(&"phantetai.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"toanazaria.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"duonggiang.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"huntanselm.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"danghuy.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"raumaxuan.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"thuyduon.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"dangdiem.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"tanquang88.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyenkien.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyenxuan77.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"hongphuct8.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"0455.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"haiga.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyentoang.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyenhien.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"sonnam.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"giaphuc.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"tramanhy7.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"khanhvang.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"myhanh.near".to_string().try_into().unwrap());
        self.contributor_10.push(&"nguyentuang.near".to_string().try_into().unwrap());
    }


    #[payable]
    pub fn init_whitelist_5(
        &mut self,
    ) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );

        self.contributor_14.push(&"nguyenvang.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"haixao.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"phnaquan.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"syxhung94.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"31be25acfbfd24cff973a1b680901476e3386593ba5290e57edba1d7965d4cbf".to_string().try_into().unwrap());
        self.contributor_14.push(&"cperdomo636.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"safroll.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"votinh2001.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"2e0c3bb76878a5ec13668fb678ae2a3ff656b50c737b38a8755a5671781acdc2".to_string().try_into().unwrap());
        self.contributor_14.push(&"433cbcee56be722783125b748256f9edbbcfb57407f2b5e32daa143df5f6e2ea".to_string().try_into().unwrap());
        self.contributor_14.push(&"lfg.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"ff59378e502ff294901a3f9949388254ea01df939aa9f3818b9488d403e8ab0d".to_string().try_into().unwrap());
        self.contributor_14.push(&"99de521082eb47a9dd82d70ec10dd76ecf9bc6c6883c180850199043023a5db8".to_string().try_into().unwrap());
        self.contributor_14.push(&"f88c4e4fdb1a76486e42dbcc1ee9bcc2b7132eeedbe9f5b85a24db2e89662b62".to_string().try_into().unwrap());
        self.contributor_14.push(&"fee632a07e682d3bb1b3191ef3ed2c86ec44bf6f29097132cb6fd9c734513f7e".to_string().try_into().unwrap());
        self.contributor_14.push(&"07a50cb4d9d5413b3582b4b6466ca5b2bba774557f01567e1ebe1c21bb676a9f".to_string().try_into().unwrap());
        self.contributor_14.push(&"jomumma.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"ff882f4b0a3697338c09763cb23f1b45ee6b47d81e173fd2a580a2363353ea6b".to_string().try_into().unwrap());
        self.contributor_14.push(&"ed670442d99b6ff61a3a2602042e7b2e976f0dd24bd60b2de2e77652212f6057".to_string().try_into().unwrap());
        self.contributor_14.push(&"selinawooo.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"tetitiaannn.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"mirandababaa.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"8c040a79a2a781b8da289a209d27d7e996ff502ef6fdaeafd52eac00fa6a9fb8".to_string().try_into().unwrap());
        self.contributor_14.push(&"mariaphanh.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"victorbeckhan.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"clintonhan.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"c65bb6cb129c38110c9a2d4b1d901edbee0853a6355b430d1e672bb31cda0869".to_string().try_into().unwrap());
        self.contributor_14.push(&"haicorbin.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"stareliass.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"lunafinne.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"desperato.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"warrotis.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"quangrory.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"vinhsilas.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"7f5b9688e3ad3288a8d35c584e39ec6a7666883af6eb4c2080cec1b0075ed073".to_string().try_into().unwrap());
        self.contributor_14.push(&"0529e6f62c6a9bac801e3e3c54e27b332724a96f3cc8f74878882c1722e21bcf".to_string().try_into().unwrap());
        self.contributor_14.push(&"79a0bef7c39104ce6ba27f0b2f5cc898ecaca1e45ba663df3a3db04ba40b91de".to_string().try_into().unwrap());
        self.contributor_14.push(&"holazaneo.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"huntan.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"azaria109.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"nguyendustin.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"a6051ebfb9685a4301301dcbd9fd3799d36b95ee2a462beba32360152c165031".to_string().try_into().unwrap());
        self.contributor_14.push(&"d5f17244d14527ad8c8f3c5f065db694f6bf872b99115b3e7d5436692bcc57fc".to_string().try_into().unwrap());
        self.contributor_14.push(&"c654c31080ae1d9119bb71e61f52f39c6de3949ffbb2f143edbdf308c9e1c4fd".to_string().try_into().unwrap());
        self.contributor_14.push(&"d5fb33179eab833c625b5bed93106229a84e8f1c09e7942811899f47fd85f7a0".to_string().try_into().unwrap());
        self.contributor_14.push(&"wagminagmi.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"cc22d8e8d27b4f2f3b18b1a10fb056a12117a939e12f5d6701f9d11b738402c7".to_string().try_into().unwrap());
        self.contributor_14.push(&"ee1bdb163faf5595e85ddd734e6b70bab01c4fe33c969be40212588e1557e759".to_string().try_into().unwrap());
        self.contributor_14.push(&"b23062ac236542a3712e4c3578355fc496867a5404b81d11ce48b9786c1a77f8".to_string().try_into().unwrap());
        self.contributor_14.push(&"62d25537fa90475defd38a53b2493459f3bde12ad2508b4666f7f3fd29f7c4c5".to_string().try_into().unwrap());
        self.contributor_14.push(&"hottieboss_91.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"4e0bd742531fa0e2c65e168c8c5de0f1b9dd6b56fd6e41dec63475cbe3ccd64e".to_string().try_into().unwrap());
        self.contributor_14.push(&"thanhdz.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"017f7f18dfe0cae4d87d0c1dda410f64412bbaa8c5ed5aebdbd98d8af7f71a6d".to_string().try_into().unwrap());
        self.contributor_14.push(&"nulesmallz.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"d814af10adc8d9425db2b36a63e56558884949600d44e36043decddfc28d5e2b".to_string().try_into().unwrap());
        self.contributor_14.push(&"mhmtnear.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"doings.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"kunaldawar.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"optimist01.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"566de10660bce23051cbc41eb389f5d6cd4555137fd54fa223b871c91a4caa05".to_string().try_into().unwrap());
        self.contributor_14.push(&"0399b405ff346c776199547badcbc7f3c5ddec112ed35e42a36e49e0e875a703".to_string().try_into().unwrap());
        self.contributor_14.push(&"7935ca179755240a1473cef485f0764750aa0f6051625167aea30d9a4144f831".to_string().try_into().unwrap());
        self.contributor_14.push(&"f487c33246878ce66eb66c7b4da69c622f1b168681607df5beac7b08a85eb98c".to_string().try_into().unwrap());
        self.contributor_14.push(&"af2a679fa33b01dfc7aac785f1a34bd97d19e7c9b67f1b8ae67f5cac977fc0ac".to_string().try_into().unwrap());
        self.contributor_14.push(&"9b36e51d4757b7767449c5b5eb6d7aefb4afe9ae02dc8eaa86f3933a0eb0f905".to_string().try_into().unwrap());
        self.contributor_14.push(&"74be53e3a431e9e69e96bf6111271e4fbd59e86625dbbcb626ca14a43f38e47b".to_string().try_into().unwrap());
        self.contributor_14.push(&"34315dd81ef4b112e79e94a3cb20c9daeb53b3a1ced299e9763e816549c057eb".to_string().try_into().unwrap());
        self.contributor_14.push(&"45a089f169c01a1d7987770c7196aec50f084cc83b53be85914a779c184ccb99".to_string().try_into().unwrap());
        self.contributor_14.push(&"0f97d74041fe38a8994739a3249e4a1cf3a66a98ff7ac796f480cece438bf0f5".to_string().try_into().unwrap());
        self.contributor_14.push(&"abilityequere.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"65e6e31fe703e88dab0539671874ad5fc5c58794a287328ff50fe533cfe05a09".to_string().try_into().unwrap());
        self.contributor_14.push(&"b33139fc5c8a7bcbdd6ea918a6bc83cc4747264431b3d2da205814ca26eca926".to_string().try_into().unwrap());
        self.contributor_14.push(&"fe13382f91f4f2c7e404a70b35a232ad0470d327b4e5efe997a186a8746c5156".to_string().try_into().unwrap());
        self.contributor_14.push(&"d466680ce9919e97817c70070eaf1261eb0ba5ae7209ee27ac52bbb6907d1078".to_string().try_into().unwrap());
        self.contributor_14.push(&"8f2a7399062bf23cfdf5eba5a947ef7cbe646ab08698dd001167211ae9606761".to_string().try_into().unwrap());
        self.contributor_14.push(&"abcbf2981949f0ef3aaa52816a4cb57fae454f8ad36ec395ae6299bb375d2bdd".to_string().try_into().unwrap());
        self.contributor_14.push(&"6f2273755b7c924ded5154ffa0fe13610f0cb27688211cd84c55ac881caded87".to_string().try_into().unwrap());
        self.contributor_14.push(&"2318c77f15be16ca0d3514f82a84fce0085c14acb66bbbc2eac83af034b07f76".to_string().try_into().unwrap());
        self.contributor_14.push(&"b6c92d57c08bb748b9c230445679dbdf378a918d9fd5e6ad37335de528195348".to_string().try_into().unwrap());
        self.contributor_14.push(&"7f0a89e99530c6c4ec3713df45c05f0411af6e9bfca50175397fb618a30a3251".to_string().try_into().unwrap());
        self.contributor_14.push(&"23dafbb0d0165f177854bb851b527c481fef6a09cd878e8433feb79432d95f19".to_string().try_into().unwrap());
        self.contributor_14.push(&"f23740d239fc8d095e1e7b40991f347600c952b5ac557d56ca15390aae95d4fe".to_string().try_into().unwrap());
        self.contributor_14.push(&"57aa58ab919354e2722798d1d8a7e0804fe9dc79e5136b45eb00751286fa0269".to_string().try_into().unwrap());
        self.contributor_14.push(&"e5e325e53ab00dbd7d6a753d2053e322353330ed5a71f30dbe429f0205988890".to_string().try_into().unwrap());
        self.contributor_14.push(&"123a0a2d4b19e337b07682dbefc03090df4929828160620959c4bf060f46c3a6".to_string().try_into().unwrap());
        self.contributor_14.push(&"b1a67f62393f78ea39a8b48ab1439a980a4493e50998d9d055116c14ebfcbd67".to_string().try_into().unwrap());
        self.contributor_14.push(&"41dbba4d22aab5495815a4d3561d6bf111e61cb564aa3286e48d546eea17d362".to_string().try_into().unwrap());
        self.contributor_14.push(&"3c0ed2b5e04d6115ba6fd83e9371607d3b108b844d3540c8bfd50d7af7809ca1".to_string().try_into().unwrap());
        self.contributor_14.push(&"7c0c39baac222046089a9789bb055a95a1a8e915c32744e2304e93680fa39357".to_string().try_into().unwrap());
        self.contributor_14.push(&"b469b8ba29151a6993b0ca145b347f0513432bca9469941c0bb31279addedf46".to_string().try_into().unwrap());
        // self.contributor_14.push(&"addr1q9r8k4vpxc4xsvxaulvq9ekq3pr495a4z2dnvcukaullwh82r76ywtac7zcdk2kt3yaw7glapzk4rjdn9psxvdmctfaqusdu7q".to_string().try_into().unwrap());
        self.contributor_14.push(&"ddfc2306d9b62926f341caf2dea494bdb60a6d4b764579631f5ecd1c8337cbf9".to_string().try_into().unwrap());
        self.contributor_14.push(&"eacb42b69f6117876a714fdd586ba05ac6382674f16627333edb626ec38448a5".to_string().try_into().unwrap());
        self.contributor_14.push(&"helix55boi.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"05f04bb0c3dc9a15a5cccce0fe7460f4c34fe38ea2e17a9480de90273cca1355".to_string().try_into().unwrap());
        self.contributor_14.push(&"02b8aace4503ea25f7e36dbb187a75ef071d69c059999bf15470d7fb1cebec06".to_string().try_into().unwrap());
        self.contributor_14.push(&"b37d1c01ada203c2e01d3b3d066e47f7984d986a6282d33f0c121499f8b43c24".to_string().try_into().unwrap());
        self.contributor_14.push(&"3287b296c94d5c147196f3af7eb036d7d64cd5b96dcedd432bd2252c06ab48f0".to_string().try_into().unwrap());
        self.contributor_14.push(&"5d910328c8b30ef86ec99c230205554373d4f08a7d8b5435b8a7580548764522".to_string().try_into().unwrap());
        self.contributor_14.push(&"121de9e4c14775222f06d4b56bc69697d1209549cb9c69bbedf87e721b1bad13".to_string().try_into().unwrap());
        self.contributor_14.push(&"9753f0c6eca80f5e4b34a03913451aead41a4480b4d39b792439157881c9e908".to_string().try_into().unwrap());
        self.contributor_14.push(&"64db20d4d0d3488478777de5af3fe79a79e0d32358c14e6979414f7f2d05469f".to_string().try_into().unwrap());
        self.contributor_14.push(&"a32a60d94f6cee01c2fe07c660b920d1e5048f650a03e5f50433f4800d79879c".to_string().try_into().unwrap());
        self.contributor_14.push(&"f63206369411de84b499350afc9ffa62e9f8d3273cc24a64117414e3fa8d6e81".to_string().try_into().unwrap());
        self.contributor_14.push(&"db28d822ac76eba0be7c61703aa810a5efbe602ff023606993c597c4a807ae79".to_string().try_into().unwrap());
        self.contributor_14.push(&"4a206902b21ecaae4152805edde4c10e8a4a7c711066fabd459dde4c14711a91".to_string().try_into().unwrap());
        self.contributor_14.push(&"73e8072f9a45b442ede0583aa76a1e3e7d33cfb2fe7f789935f12c1488185e2d".to_string().try_into().unwrap());
        self.contributor_14.push(&"15aeec9c3587e58f4e4f7459c399375de7022761b3cdcd7a4b9f647ed5136268".to_string().try_into().unwrap());
        self.contributor_14.push(&"7c8526e2ca5bd8739f40641c800898d56c167f86a37af75ec67219819fb7f569".to_string().try_into().unwrap());
        self.contributor_14.push(&"f3d813c6850f1f19399f3c080932f2f07dbe9fdefec9fc043584981622901baf".to_string().try_into().unwrap());
        self.contributor_14.push(&"d79275418ecf8016463075109ab3ab500618eaf19a212277b368ed6740579a9a".to_string().try_into().unwrap());
        self.contributor_14.push(&"b0c975a26ea141a1a9f4c3be68ed394aca4b82c68f0909b420ddda1f6a94affd".to_string().try_into().unwrap());
        self.contributor_14.push(&"4e48a3ce2dfbdffae9cb4e952d698992cebe011d9a10baeff960315183e1a85e".to_string().try_into().unwrap());
        self.contributor_14.push(&"adrian83.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"00fde568a34542394e79b2b53a3fcb63f34eda206350668f2b568c82a325d5e8".to_string().try_into().unwrap());
        self.contributor_14.push(&"6aaddd926030799d609ab4b3986c86d86ce57d28563f388d6b9d974baa301bb4".to_string().try_into().unwrap());
        self.contributor_14.push(&"rugmedaddy101.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"e30e7ed5173074dcb38889e0ae906618cc8d47ab94851b34db8d987b4d1a1c37".to_string().try_into().unwrap());
        self.contributor_14.push(&"79c84e32b4fe9698b8c02981eafdd5af5033706bafaf1db6d5af86f08517fb86".to_string().try_into().unwrap());
        self.contributor_14.push(&"th16032511.near".to_string().try_into().unwrap());
        // self.contributor_14.push(&"letsgetnaked.near ".to_string().try_into().unwrap());
        self.contributor_14.push(&"pemmiee.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"riqi.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"gudangfidel.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"afiqshofy.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"arezhas.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"grantevns.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"980360e47f9c2a1383b178eb65567e3fe0e9787aa43c664e71c64a9389f78dfb".to_string().try_into().unwrap());
        self.contributor_14.push(&"171dc581bfd3b52d766dbc2f115d2a2e764ad690b29dc1bdf2293998905e5523".to_string().try_into().unwrap());
        self.contributor_14.push(&"cryptok2adir.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"lunes.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"4d8916b547519bf418cf5dc7af917ba6ef77c5a6c7ad23a07510e83508baa995".to_string().try_into().unwrap());
        self.contributor_14.push(&"c83424313985b941b98b9bb22cd289ab961f36bb643dcda589cb9be0183db680".to_string().try_into().unwrap());
        self.contributor_14.push(&"56280845019113c8bf3fe28d1f3d7c425be383a6defe5b703100d9a32f0c1d39".to_string().try_into().unwrap());
        self.contributor_14.push(&"letrung0001.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"dbf17c169f66ab79fb1403681fe94d6a6a254b3001039b48f46a03e5e59770db".to_string().try_into().unwrap());
        self.contributor_14.push(&"lenguyenthienanh.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"arale.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"braimah7.near".to_string().try_into().unwrap());
        self.contributor_14.push(&"nftkin.near".to_string().try_into().unwrap());
    }
}
