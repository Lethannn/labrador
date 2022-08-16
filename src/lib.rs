// Copyright (c) 2022 Labrador contributors
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! This create offers:
//!
//! *   A convenient mainstream third-party service client
//! *   Convenient and quick use of corresponding services in rust
//!
//! Features:
//!
//! *   ```taobao``` - Taobao customer related services
//! *   ```alipay``` - Alipay related services
//! *   ```pdd``` - Pinduoduo related services
//! *   ```jd``` - Jingdong related services
//! *   ```wechat``` - Wechat related services
//!
//! ## Installation
//!
//! Put the desired version of the crate into the `dependencies` section of your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! labrador = "0.1.0"
//! ```
//!
//!
//!
//! ## Example
//!
//! ### With Wechat
//!
//! ```rust
//! # use labrador::{WeChatPayClient, SimpleStorage, TradeType, WeChatPayRequestV3, Amount, Payer};
//! # use chrono::{Local, SecondsFormat};
//!
//! #[tokio::main]
//! async fn main() {
//!     let c =  WeChatPayClient::new("appid", "secret", SimpleStorage::new());
//!     let mut client =c.wxpay();
//!     let date = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
//!     let result = client.unified_order_v3(TradeType::Jsapi, WeChatPayRequestV3 {
//!         appid: "appid".to_string().into(),
//!         mch_id: "mchid".to_string(),
//!         description: "测试商品支付".to_string(),
//!         out_trade_no: "1602920235sdfsdfas32234234".to_string(),
//!         time_expire: date,
//!         attach: None,
//!         notify_url: "https://!xxx.cn/trade/notify".to_string(),
//!         amount: Amount {
//!             total: 1,
//!             currency: String::from("CNY").into(),
//!             payer_total: None,
//!             payer_currency: None
//!         },
//!         payer: Payer {
//!             openid: "oUVZc6S_uGx3bsNPUA-davo4Dt7Us".to_string()
//!         }.into(),
//!         detail: None,
//!         scene_info: None,
//!         settle_info: None
//!     });
//!     match result.await {
//!         Ok(res) => {}
//!         Err(err) => {}
//!     }
//! }
//! ```
//!
//!
mod session;
mod request;
mod errors;
mod client;
mod util;
#[cfg(feature = "jd")]
mod jd;
#[cfg(feature = "jd")]
pub use jd::*;
#[cfg(feature = "taobao")]
mod taobao;
#[cfg(feature = "taobao")]
pub use taobao::*;
#[cfg(feature = "pdd")]
mod pdd;
#[cfg(feature = "pdd")]
pub use pdd::*;
#[cfg(feature = "wechat")]
mod wechat;
#[cfg(feature = "wechat")]
pub use wechat::*;

pub type LabradorResult<T, E = LabraError> = Result<T, E>;

#[cfg(feature = "alipay")]
mod alipay;
#[cfg(feature = "alipay")]
pub use alipay::*;

pub use errors::LabraError;
pub use session::*;
pub use util::*;
pub use client::APIClient;
pub use request::*;

pub use bytes;
pub use serde_urlencoded;
pub use urlencoding;
pub use dashmap;