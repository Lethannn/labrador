# Labrador &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![labrador: rustc 1.13+]][Rust 1.13]

[Build Status]: https://img.shields.io/docsrs/labrador/0.1.0?style=plastic
[actions]: https://github.com/wslongchen/labrador/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/labrador?style=plastic
[crates.io]: https://crates.io/crates/labrador
[labrador: rustc 1.13+]: https://img.shields.io/badge/labrador-rustc__1.31%2B-lightgrey
[Rust 1.13]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

```Labrador - Mini client for rust ```


# This create offers:

*   A convenient mainstream third-party service client
*   Convenient and quick use of corresponding services in rust

Features:

*   ```taobao``` - Taobao customer related services
*   ```alipay``` - Alipay related services
*   ```pdd``` - Pinduoduo related services
*   ```jd``` - Jingdong related services
*   ```wechat``` - Wechat related services


---

You may be looking for:

- [An overview of Labrador](https://crates.io/crates/labrador)
- [Examples](https://github.com/wslongchen/labrador/blob/0.1.0/example/simple.rs)
- [API documentation](https://docs.rs/labrador/0.1.0/labrador/)
- [Release notes](https://github.com/wslongchen/labrador/releases)

## Labrador in action

<details>
<summary>
Click to show Cargo.toml.
<a href="https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=93bca9fced54f62eb69a2f2a224715c5" target="_blank">Run this code in the playground.</a>
</summary>

```toml
[dependencies]

# The core APIs
labrador = { version = "0.1.0", features = ["wechat", "alipay"] }

```

</details>
<p></p>

## API Documentation

## Example

### With Wechat

 ```rust
use labrador::{WeChatPayClient, SimpleStorage, TradeType, WeChatPayRequestV3, Amount, Payer};
use chrono::{Local, SecondsFormat};

 #[tokio::main]
 async fn main() {
     let c =  WeChatPayClient::new("appid", "secret", SimpleStorage::new());
     let mut client =c.wxpay();
     let date = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
     let result = client.unified_order_v3(TradeType::Jsapi, WeChatPayRequestV3 {
         appid: "appid".to_string().into(),
         mch_id: "mchid".to_string(),
         description: "测试商品支付".to_string(),
         out_trade_no: "1602920235sdfsdfas32234234".to_string(),
         time_expire: date,
         attach: None,
         notify_url: "https:xxx.cn/trade/notify".to_string(),
         amount: Amount {
             total: 1,
             currency: String::from("CNY").into(),
             payer_total: None,
             payer_currency: None
         },
         payer: Payer {
             openid: "oUVZc6S_uGx3bsNPUA-davo4Dt7Us".to_string()
         }.into(),
         detail: None,
         scene_info: None,
         settle_info: None
     });
     match result.await {
         Ok(res) => {}
         Err(err) => {}
     }
 }
 ```

## Developing

To setup the development envrionment run `cargo run`.

## Contributers

	MrPan <1049058427@qq.com>

## Getting help

Labrador is a personal project. At the beginning, I just like Labrador dog because of my hobbies.
I hope this project will grow more and more lovely. Many practical database functions will
be added in the future. I hope you can actively help this project grow and put forward suggestions.
I believe the future will be better and better.

[#general]: https://discord.com/channels/273534239310479360/274215136414400513
[#beginners]: https://discord.com/channels/273534239310479360/273541522815713281
[#rust-usage]: https://discord.com/channels/442252698964721669/443150878111694848
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/122651-general
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust
[/r/rust]: https://www.reddit.com/r/rust
[discourse]: https://users.rust-lang.org

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Labrador by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>