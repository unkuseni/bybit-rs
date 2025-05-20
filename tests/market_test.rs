use bybit::prelude::*;

use tokio::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    use tokio::test;

    #[test]
    async fn test_kline() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let klines = market.get_klines(request).await;
        if let Ok(data) = klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_mark_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let mark_klines = market.get_mark_price_klines(request).await;
        if let Ok(data) = mark_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_index_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let index_klines = market.get_index_price_klines(request).await;
        if let Ok(data) = index_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_premium_klines() {
        let market: MarketData = Bybit::new(None, None);
        let request = KlineRequest::new(
            Some(Category::Linear),
            "ETHUSDT",
            "60",
            Some("010124"),
            Some("050224"),
            None,
        );
        let premium_klines = market.get_premium_index_price_klines(request).await;
        if let Ok(data) = premium_klines {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_futures_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Linear, Some("ETHUSDT"), None, None, None);
        let instrument = market.get_instrument_info(request).await;
        if let Ok(data) = instrument {
            match data.result {
                InstrumentInfo::Futures(futures) => println!("{:#?}", futures.list[0]),
                _ => println!("not futures"),
            }
        }
    }

    #[test]
    async fn test_spot_instrument() {
        let market: MarketData = Bybit::new(None, None);
        let request = InstrumentRequest::new(Category::Spot, Some("ETHUSDT"), None, None, None);
        let instrument = market.get_instrument_info(request).await;
        if let Ok(data) = instrument {
            match data.result {
                InstrumentInfo::Spot(spot) => println!("{:#?}", spot.list[0]),
                _ => println!("not spot"),
            }
        }
    }

    #[test]
    async fn test_market() {
        let market: MarketData = Bybit::new(None, None);
        let five_minutes = Duration::from_secs(5 * 60);
        let request = OrderbookRequest::new("ETHUSDT", Category::Linear, Some(1));
        let start = Instant::now();
        while Instant::now() - start < five_minutes {
            let order_book = market.get_depth(request.clone()).await;
            if let Ok(data) = order_book {
                let order_book = data.result;
                let mid_price = (order_book.asks[0].price + order_book.bids[0].price) / 2.0;
                let imbalance = (order_book.bids[0].qty - order_book.asks[0].qty)
                    / (order_book.asks[0].qty + order_book.bids[0].qty);
                let fees = fee_percent(mid_price, 0.04);
                let spread = order_book.asks[0].price - order_book.bids[0].price;
                let arb = spread - fees;
                println!(
                    "{:#?} , Spread: {:.5} Arb: {} Imb: {:.4}",
                    order_book,
                    spread,
                    if arb > fee_percent(mid_price, 0.02) {
                        arb
                    } else {
                        0.0
                    },
                    imbalance
                );
            }
        }
    }

    fn fee_percent(value: f64, percent: f64) -> f64 {
        (percent / 100.0) * value
    }

    #[test]
    async fn test_futures_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETHUSDT";
        let futures_ticker = market.get_tickers(Some(symbol), Category::Linear).await;
        if let Ok(data) = futures_ticker {
            match &data.result.list[0] {
                TickerData::Futures(futures) => println!("{:#?}", futures),
                _ => println!("not futures"),
            }
        }
    }

    #[test]
    async fn test_spot_ticker() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETHUSDT";
        let spot_ticker = market.get_tickers(Some(symbol), Category::Spot).await;
        if let Ok(data) = spot_ticker {
            match &data.result.list[0].clone() {
                TickerData::Spot(spot) => println!("{:#?}", spot),
                _ => println!("not spot"),
            }
        }
    }

    #[test]
    async fn test_recent_trades() {
        let market: MarketData = Bybit::new(None, None);
        let request = RecentTradesRequest::new(Category::Linear, Some("POLUSDT"), None, None);
        let trades = market.get_recent_trades(request).await;
        if let Ok(data) = trades {
            println!("{:#?}", data.result.list);
        }
    }

    #[test]
    async fn test_funding_rate() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let request = FundingHistoryRequest::new(Category::Linear, symbol, None, None, None);
        let funding_rate = market.get_funding_history(request).await;
        if let Ok(data) = funding_rate {
            println!("{:#?}", data.result.list.last().unwrap());
        }
    }

    #[test]
    async fn test_open_interest() {
        let market: MarketData = Bybit::new(None, None);
        let request = OpenInterestRequest::new(Category::Linear, "ETHUSDT", "1h", None, None, None);
        let open_interest = market.get_open_interest(request).await;
        if let Ok(data) = open_interest {
            println!("{:#?}", data.result.list.last().unwrap());
        }
    }

    #[test]
    async fn test_historical_volatility() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "ETH";
        let request: HistoricalVolatilityRequest<'_> =
            HistoricalVolatilityRequest::new(Some(Cow::Borrowed(symbol)), None, None, None);
        let historical_volatility = market.get_historical_volatility(request).await;
        if let Ok(data) = historical_volatility {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_insurance() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = Some("BTC");
        let insurance = market.get_insurance(symbol).await;
        if let Ok(data) = insurance {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_risk_limit() {
        let market: MarketData =
            Bybit::new_with_config(&Config::default().set_recv_window(1000), None, None);
        let symbol = "ETHUSDT";
        let request: RiskLimitRequest<'_> = RiskLimitRequest::new(Category::Linear, Some(symbol));
        let risk_limit = market.get_risk_limit(request).await;
        if let Ok(data) = risk_limit {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_delivery_price() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let delivery_price = market
            .get_delivery_price(Category::Option, Some(symbol), None, None)
            .await;
        if let Ok(data) = delivery_price {
            println!("{:#?}", data.result);
        }
    }

    #[test]
    async fn test_longshort_ratio() {
        let market: MarketData = Bybit::new(None, None);
        let symbol = "BTCUSDT";
        let longshort_ratio = market
            .get_longshort_ratio(Category::Linear, symbol, "4h", None)
            .await;
        if let Ok(data) = longshort_ratio {
            println!("{:#?}", data.result);
        }
    }
}
