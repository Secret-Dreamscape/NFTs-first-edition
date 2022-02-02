use cosmwasm_std::{Coin, Extern, from_binary, HandleResult, InitResponse, StdResult, Uint128};
use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};

#[cfg(test)]
mod tests {
  use cosmwasm_std::HumanAddr;

  use secret_dreamscape_nfts::contract::{handle, init};
  use secret_dreamscape_nfts::msg::{HandleMsg, InitConfig, InitMsg};
  use secret_dreamscape_nfts::royalties::{Royalty, RoyaltyInfo};
  use secret_dreamscape_nfts::state::PreLoad;
  use secret_dreamscape_nfts::token::Trait;

  use super::*;

  fn init_contract() -> (
    StdResult<InitResponse>,
    Extern<MockStorage, MockApi, MockQuerier>,
  ) {
    let mut deps = mock_dependencies(20, &[]);
    let env = mock_env("user0", &[]);

    let init_msg = InitMsg {
      name: "Secret Dreamscape Card".to_string(),
      symbol: "SCRTDREAM".to_string(),
      admin: Some(HumanAddr("user0".to_string())),
      entropy: "test".to_string(),
      royalty_info: None,
      config: Some(InitConfig {
        public_token_supply: Some(true),
        public_owner: Some(false),
        enable_sealed_metadata: Some(false),
        unwrapped_metadata_is_private: Some(true),
        minter_may_update_metadata: Some(true),
        owner_may_update_metadata: Some(false),
        enable_burn: Some(true),
      }),
      post_init_callback: None,
      mint_funds_distribution_info: Some(RoyaltyInfo {
        decimal_places_in_rates: 4,
        royalties: vec![Royalty {
          recipient: HumanAddr("user0".to_string()),
          rate: 20,
        }],
      }),
    };

    (init(&mut deps, env, init_msg), deps)
  }

  pub fn msg(
    deps: &mut Extern<MockStorage, MockApi, MockQuerier>,
    msg: HandleMsg,
  ) -> HandleResult {
    handle(
      deps,
      mock_env(
        "user0".to_string(),
        &[],
      ),
      msg,
    )
  }

//  #[test]
//  fn test_1() {
//    let (_, mut deps) = init_contract();
//
//    msg(&mut deps, HandleMsg::PreLoad {
//      new_data: vec![
//        PreLoad {
//          id: "QmWsS5EBMuwerpL919nXtKcGJ6chbsGvq6MJiKL9pi7FYw".to_string(),
//          img_url:
//          "ipfs://QmWsS5EBMuwerpL919nXtKcGJ6chbsGvq6MJiKL9pi7FYw".to_string(),
//          priv_img_url:
//          "ipfs://QmWsS5EBMuwerpL919nXtKcGJ6chbsGvq6MJiKL9pi7FYw".to_string(),
//          attributes: Some(vec![
//            Trait {
//              display_type: Some("Letter".to_string()),
//              trait_type: Some("letter".to_string()),
//              value: "A".to_string(),
//              max_value: None,
//            },
//            Trait {
//              display_type: Some("Gold".to_string()),
//              trait_type: Some("gold".to_string()),
//              value: "true".to_string(),
//              max_value: None,
//            },
//          ]),
//          priv_attributes: Some(vec![]),
//          priv_key: "QmWsS5EBMuwerpL919nXtKcGJ6chbsGvq6MJiKL9pi7FYw".to_string(),
//        }
//      ]
//    });
//
//    let result = msg(&mut deps, HandleMsg::Mint {
//      count: 1
//    });
//
//    assert!(result.is_ok(), "mint wasn't successful");
//  }
}
