#![allow(unused_imports, unreachable_code, unused_variables)]
use std::collections::BTreeMap;

use serde_json::Value;
use crate::api::{API, Account};
use crate::client::Client;
use crate::errors::Result;

use crate::model::*;
use crate::util::{build_json_request, build_request};


#[derive(Clone)]
pub struct AccountManager {
  pub client: Client,
  pub recv_window: u64,
}

impl AccountManager {
    pub async fn get_wallet_balance<'a>(&self, req: WalletRequest<'a>) -> Result<Value> {
     let mut parameters: BTreeMap<String, Value> = BTreeMap::new(); 
     parameters.insert("accountType".into(), req.account_type.into());
     if let Some(c) = req.coin {
        parameters.insert("coin".into(), c.into());
     }
     let request = build_request(&parameters);
     let _response: Value = self
        .client
        .get_signed(API::Account(Account::Balance), self.recv_window.into(), Some(request))
        .await?;
      todo!("This endpoint has issues authenticating signatures" );
     Ok(_response)
    }

    pub async  fn get_account_info(&self) -> Result<Value> {
        let response: Value = self
           .client
           .get_signed(API::Account(Account::Information), self.recv_window.into(), None)
           .await?;
        Ok(response)
    } 

    pub async fn get_collateral_info(&self, coin: String) -> Result<Value> {
      let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
      parameters.insert("currency".into(), coin.into());
      let req  = build_request(&parameters);
        let _response: Value = self
           .client
           .get_signed(API::Account(Account::CollateralInfo), self.recv_window.into(), Some(req))
           .await?;
          todo!("This endpoint has issues authenticating signatures" );
        Ok(_response)
    }

    pub async fn get_fee_rate(&self, category: Category, symbol: Option<String>) -> Result<Value> {
      let mut parameters: BTreeMap<String, Value> = BTreeMap::new();
      if let Some(s) = symbol {
        parameters.insert("symbol".into(), s.into());
      }
      let req  = build_request(&parameters);
        let _response: Value = self
           .client
           .get_signed(API::Account(Account::FeeRate), self.recv_window.into(), Some(req))
           .await?;
          todo!("This endpoint has issues authenticating signatures" );
        Ok(_response)
    }

}