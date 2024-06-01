use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::accounts::AccountsInstrument;
use super::order::ComplexOrderStrategyType;
use super::order::Duration;
use super::order::Order;
use super::order::OrderActivity;
use super::order::OrderLegCollection;
use super::order::OrderStrategyType;
use super::order::OrderType;
use super::order::PriceLinkBasis;
use super::order::PriceLinkType;
use super::order::Session;
use super::order::SpecialInstruction;
use super::order::Status;
use super::order::StopPriceLinkBasis;
use super::order::StopPriceLinkType;
use super::order::StopType;
use super::order::TaxLotMethod;
use super::preview_order::Instruction;
use crate::model::market_data::instrument::InstrumentAssetType;
use crate::model::InstrumentResponse;
use crate::Error;

/// More Info in [Charles Schwab Developer Portal](https://developer.schwab.com/) : API Products -> Trader API - Individual -> Accounts and Trading Production -> Documentation -> Place Order Samples
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option), default)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub session: Option<Session>,
    pub duration: Option<Duration>,
    pub order_type: Option<OrderTypeRequest>,
    pub cancel_time: Option<chrono::DateTime<chrono::Utc>>,
    pub complex_order_strategy_type: Option<ComplexOrderStrategyType>,
    pub quantity: Option<f64>,
    pub filled_quantity: Option<f64>,
    pub remaining_quantity: Option<f64>,
    pub destination_link_name: Option<String>,
    pub release_time: Option<chrono::DateTime<chrono::Utc>>,
    pub stop_price: Option<f64>,
    pub stop_price_link_basis: Option<StopPriceLinkBasis>,
    pub stop_price_link_type: Option<StopPriceLinkType>,
    pub stop_price_offset: Option<f64>,
    pub stop_type: Option<StopType>,
    pub price_link_basis: Option<PriceLinkBasis>,
    pub price_link_type: Option<PriceLinkType>,
    pub price: Option<f64>,
    pub tax_lot_method: Option<TaxLotMethod>,
    /// xml: OrderedMap { "name": "orderLegCollection", "wrapped": true }
    pub order_leg_collection: Option<Vec<OrderLegCollectionRequest>>,
    pub activation_price: Option<f64>,
    pub special_instruction: Option<SpecialInstruction>,
    pub order_strategy_type: OrderStrategyType,
    pub order_id: Option<i64>,
    /// default: false
    pub cancelable: Option<bool>,
    /// default: false
    pub editable: Option<bool>,
    pub status: Option<Status>,
    pub entered_time: Option<chrono::DateTime<chrono::Utc>>,
    pub close_time: Option<chrono::DateTime<chrono::Utc>>,
    pub account_number: Option<i64>,
    /// xml: OrderedMap { "name": "orderActivity", "wrapped": true }
    pub order_activity_collection: Option<Vec<OrderActivity>>,
    /// xml: OrderedMap { "name": "replacingOrder", "wrapped": true }
    pub replacing_order_collection: Option<Vec<String>>,
    /// xml: OrderedMap { "name": "childOrder", "wrapped": true }
    pub child_order_strategies: Option<Vec<OrderRequest>>,
    pub status_description: Option<String>,
}

impl From<Order> for OrderRequest {
    fn from(value: Order) -> Self {
        Self {
            session: Some(value.session),
            duration: Some(value.duration),
            order_type: Some(value.order_type.into()),
            cancel_time: value.cancel_time,
            complex_order_strategy_type: Some(value.complex_order_strategy_type),
            quantity: Some(value.quantity),
            filled_quantity: Some(value.filled_quantity),
            remaining_quantity: Some(value.remaining_quantity),
            destination_link_name: Some(value.destination_link_name),
            release_time: value.release_time,
            stop_price: value.stop_price,
            stop_price_link_basis: value.stop_price_link_basis,
            stop_price_link_type: value.stop_price_link_type,
            stop_price_offset: value.stop_price_offset,
            stop_type: value.stop_type,
            price_link_basis: value.price_link_basis,
            price_link_type: value.price_link_type,
            price: Some(value.price),
            tax_lot_method: value.tax_lot_method,
            order_leg_collection: Some(
                value
                    .order_leg_collection
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            activation_price: value.activation_price,
            special_instruction: value.special_instruction,
            order_strategy_type: value.order_strategy_type,
            order_id: Some(value.order_id),
            cancelable: Some(value.cancelable),
            editable: Some(value.editable),
            status: Some(value.status),
            entered_time: Some(value.entered_time),
            close_time: value.close_time,
            account_number: Some(value.account_number),
            order_activity_collection: value.order_activity_collection,
            replacing_order_collection: value.replacing_order_collection,
            child_order_strategies: value
                .child_order_strategies
                .map(|orders| orders.into_iter().map(Into::into).collect()),
            status_description: value.status_description,
        }
    }
}

impl OrderRequest {
    /// Create a market order.
    pub fn market(
        symbol: InstrumentRequest,
        instruction: Instruction,
        quantity: f64,
    ) -> Result<Self, Error> {
        let order_leg_collection = vec![OrderLegCollectionRequest {
            instruction,
            quantity,
            instrument: symbol,
        }];
        OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Market)
            .session(Session::Normal)
            .duration(Duration::Day)
            .order_strategy_type(OrderStrategyType::Single)
            .order_leg_collection(order_leg_collection)
            .build()
            .map_err(Error::OrderRequestBuild)
    }

    /// Create a limit order.
    pub fn limit(
        symbol: InstrumentRequest,
        instruction: Instruction,
        quantity: f64,
        price: f64,
    ) -> Result<Self, Error> {
        let order_leg_collection = vec![OrderLegCollectionRequest {
            instruction,
            quantity,
            instrument: symbol,
        }];
        OrderRequestBuilder::default()
            .complex_order_strategy_type(ComplexOrderStrategyType::None)
            .order_type(OrderTypeRequest::Limit)
            .session(Session::Normal)
            .price(price)
            .duration(Duration::Day)
            .order_strategy_type(OrderStrategyType::Single)
            .order_leg_collection(order_leg_collection)
            .build()
            .map_err(Error::OrderRequestBuild)
    }
}

/// Same as `super::order::OrderType`, but does not have UNKNOWN since this type is not allowed as an input
/// Type of order to place.
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTypeRequest {
    /// Execute the order immediately at the best-available price.
    /// `More Info <https://www.investopedia.com/terms/m/marketorder.asp>`__.
    Market,
    #[default]
    /// Execute the order at your price or better.
    /// `More info <https://www.investopedia.com/terms/l/limitorder.asp>`__.
    Limit,
    /// Wait until the price reaches the stop price, and then immediately place a
    /// market order.
    /// `More Info <https://www.investopedia.com/terms/l/limitorder.asp>`__.
    Stop,
    /// Wait until the price reaches the stop price, and then immediately place a
    /// limit order at the specified price.
    /// `More Info <https://www.investopedia.com/terms/s/stop-limitorder.asp>`__.
    StopLimit,
    /// Similar to ``STOP``, except if the price moves in your favor, the stop
    /// price is adjusted in that direction. Places a market order if the stop
    /// condition is met.
    /// `More info <https://www.investopedia.com/terms/t/trailingstop.asp>`__.
    TrailingStop,
    Cabinet,
    NonMarketable,
    /// Place the order at the closing price immediately upon market close.
    /// `More info <https://www.investopedia.com/terms/m/marketonclose.asp>`__
    MarketOnClose,
    /// Exercise an option.
    Exercise,
    /// Similar to ``STOP_LIMIT``, except if the price moves in your favor, the
    /// stop price is adjusted in that direction. Places a limit order at the
    /// specified price if the stop condition is met.
    /// `More info <https://www.investopedia.com/terms/t/trailingstop.asp>`__.
    TrailingStopLimit,
    /// Place an order for an options spread resulting in a net debit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetDebit,
    /// Place an order for an options spread resulting in a net credit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetCredit,
    /// Place an order for an options spread resulting in neither a credit nor a
    /// debit.
    /// `More info <https://www.investopedia.com/ask/answers/042215/whats-difference-between-credit-spread-and-debt-spread.asp>`__
    NetZero,
    LimitOnClose,
}

impl From<OrderType> for OrderTypeRequest {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Market => OrderTypeRequest::Market,
            OrderType::Limit => OrderTypeRequest::Limit,
            OrderType::Stop => OrderTypeRequest::Stop,
            OrderType::StopLimit => OrderTypeRequest::StopLimit,
            OrderType::TrailingStop => OrderTypeRequest::TrailingStop,
            OrderType::Cabinet => OrderTypeRequest::Cabinet,
            OrderType::NonMarketable => OrderTypeRequest::NonMarketable,
            OrderType::MarketOnClose => OrderTypeRequest::MarketOnClose,
            OrderType::Exercise => OrderTypeRequest::Exercise,
            OrderType::TrailingStopLimit => OrderTypeRequest::TrailingStopLimit,
            OrderType::NetDebit => OrderTypeRequest::NetDebit,
            OrderType::NetCredit => OrderTypeRequest::NetCredit,
            OrderType::NetZero => OrderTypeRequest::NetZero,
            OrderType::LimitOnClose => OrderTypeRequest::LimitOnClose,
            OrderType::Unknown => panic!("Unknown"),
        }
    }
}

/// Similar to `super::order::OrderLegCollection`, but more simple
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollectionRequest {
    pub instrument: InstrumentRequest,
    pub instruction: Instruction,
    pub quantity: f64,
}

impl From<OrderLegCollection> for OrderLegCollectionRequest {
    fn from(value: OrderLegCollection) -> Self {
        Self {
            instrument: value.instrument.into(),
            instruction: value.instruction,
            quantity: value.quantity,
        }
    }
}

/// Similar to `super::accounts::AccountsInstrument`, but more simple
/// only support Equity, Option now in schwab API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assetType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstrumentRequest {
    Equity { symbol: String },
    Option { symbol: String },
}

impl From<AccountsInstrument> for InstrumentRequest {
    fn from(value: AccountsInstrument) -> Self {
        match value {
            AccountsInstrument::CashEquivalent(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::Equity(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::FixedIncome(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::MutualFund(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::Option(x) => Self::Option {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::Index(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::Currency(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
            AccountsInstrument::CollectiveInvestment(x) => Self::Equity {
                symbol: x.accounts_base_instrument.symbol,
            },
        }
    }
}

impl From<InstrumentResponse> for InstrumentRequest {
    fn from(value: InstrumentResponse) -> Self {
        match value.asset_type {
            InstrumentAssetType::Bond
            | InstrumentAssetType::Equity
            | InstrumentAssetType::Etf
            | InstrumentAssetType::Extended
            | InstrumentAssetType::Forex
            | InstrumentAssetType::Future
            | InstrumentAssetType::Fundamental
            | InstrumentAssetType::Index
            | InstrumentAssetType::Indicator
            | InstrumentAssetType::MutualFund
            | InstrumentAssetType::Unknown => Self::Equity {
                symbol: value.symbol,
            },
            InstrumentAssetType::FutureOption | InstrumentAssetType::Option => Self::Option {
                symbol: value.symbol,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
    use serde_json::json;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/OrderRequest.json"
        ));

        let val = serde_json::from_str::<OrderRequest>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_market() {
        // Buy Market: Stock
        // Buy 15 shares of XYZ at the Market good for the Day.
        let expected = json!({
            "orderType": "MARKET",
            "session": "NORMAL",
            "duration": "DAY",
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [
                {
                    "instruction": "BUY",
                    "quantity": 15,
                    "instrument": {
                        "symbol": "XYZ",
                        "assetType": "EQUITY"
                    }
                }
            ]
        });

        let symbol = InstrumentRequest::Equity {
            symbol: "XYZ".to_string(),
        };
        let order_req = OrderRequest::market(symbol, Instruction::Buy, 15.0).unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_limit() {
        // Buy Limit: Single Option
        // Buy to open 10 contracts of the XYZ March 15, 2024 $50 CALL at a Limit of $6.45 good for the Day.
        let expected = json!({
            "complexOrderStrategyType": "NONE",
            "orderType": "LIMIT",
            "session": "NORMAL",
            "price": 6.45,
            "duration": "DAY",
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [
                {
                    "instruction": "BUY_TO_OPEN",
                    "quantity": 10,
                    "instrument": {
                        "symbol": "XYZ   240315C00500000",
                        "assetType": "OPTION"
                    }
                }
            ]
        });

        let symbol = InstrumentRequest::Option {
            symbol: "XYZ   240315C00500000".to_string(),
        };
        let order_req = OrderRequest::limit(symbol, Instruction::BuyToOpen, 10.0, 6.45).unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_vertical_call_spread() {
        // Buy Limit: Vertical Call Spread
        // Buy to open 2 contracts of the XYZ March 15, 2024 $45 Put and Sell to open 2 contract of the XYZ March 15, 2024 $43 Put at a LIMIT price of $0.10 good for the Day.
        let expected = json!({
            "orderType": "NET_DEBIT",
            "session": "NORMAL",
            "price": 0.1,
            "duration": "DAY",
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [
                {
                    "instruction": "BUY_TO_OPEN",
                    "quantity": 2,
                    "instrument": {
                        "symbol": "XYZ   240315P00045000",
                        "assetType": "OPTION"
                    }
                },
                {
                    "instruction": "SELL_TO_OPEN",
                    "quantity": 2,
                    "instrument": {
                        "symbol": "XYZ   240315P00043000",
                        "assetType": "OPTION"
                    }
                }
            ]
        });

        let symbol1 = InstrumentRequest::Option {
            symbol: "XYZ   240315P00045000".to_string(),
        };
        let symbol2 = InstrumentRequest::Option {
            symbol: "XYZ   240315P00043000".to_string(),
        };
        let order_req = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::NetDebit)
            .session(Session::Normal)
            .duration(Duration::Day)
            .price(0.1)
            .order_leg_collection(vec![
                OrderLegCollectionRequest {
                    instruction: Instruction::BuyToOpen,
                    quantity: 2.0,
                    instrument: symbol1,
                },
                OrderLegCollectionRequest {
                    instruction: Instruction::SellToOpen,
                    quantity: 2.0,
                    instrument: symbol2,
                },
            ])
            .build()
            .unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_one_triggers_another() {
        // Conditional Order: One Triggers Another
        // Buy 10 shares of XYZ at a Limit price of $34.97 good for the Day. If filled, immediately submit an order to Sell 10 shares of XYZ with a Limit price of $42.03 good for the Day. Also known as 1st Trigger Sequence.
        let expected = json!({
            "orderType": "LIMIT",
            "session": "NORMAL",
            "price": 34.97,
            "duration": "DAY",
            "orderStrategyType": "TRIGGER",
            "orderLegCollection": [
                {
                    "instruction": "BUY",
                    "quantity": 10,
                    "instrument": {
                        "symbol": "XYZ",
                        "assetType": "EQUITY"
                    }
                }
            ],
            "childOrderStrategies": [
                {
                    "orderType": "LIMIT",
                    "session": "NORMAL",
                    "price": 42.03,
                    "duration": "DAY",
                    "orderStrategyType": "SINGLE",
                    "orderLegCollection": [
                        {
                            "instruction": "SELL",
                            "quantity": 10,
                            "instrument": {
                                "symbol": "XYZ",
                                "assetType": "EQUITY"
                            }
                        }
                    ]
                }
            ]
        });

        let symbol = InstrumentRequest::Equity {
            symbol: "XYZ".to_string(),
        };

        let child_order_req = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Limit)
            .session(Session::Normal)
            .duration(Duration::Day)
            .price(42.03)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 10.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let order_req = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Limit)
            .session(Session::Normal)
            .duration(Duration::Day)
            .price(34.97)
            .order_strategy_type(OrderStrategyType::Trigger)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Buy,
                quantity: 10.0,
                instrument: symbol,
            }])
            .child_order_strategies(vec![child_order_req])
            .build()
            .unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_one_cancels_another() {
        // Conditional Order: One Cancels Another
        // Sell 2 shares of XYZ at a Limit price of $45.97 and Sell 2 shares of XYZ with a Stop Limit order where the stop price is $37.03 and limit is $37.00. Both orders are sent at the same time. If one order fills, the other order is immediately cancelled. Both orders are good for the Day. Also known as an OCO order.
        let expected = json!({
            "orderStrategyType": "OCO",
            "childOrderStrategies": [
                {
                    "orderType": "LIMIT",
                    "session": "NORMAL",
                    "price": 45.97,
                    "duration": "DAY",
                    "orderStrategyType": "SINGLE",
                    "orderLegCollection": [
                        {
                            "instruction": "SELL",
                            "quantity": 2,
                            "instrument": {
                                "symbol": "XYZ",
                                "assetType": "EQUITY"
                            }
                        }
                    ]
                },
                {
                    "orderType": "STOP_LIMIT",
                    "session": "NORMAL",
                    "price": 37.0,
                    "stopPrice": 37.03,
                    "duration": "DAY",
                    "orderStrategyType": "SINGLE",
                    "orderLegCollection": [
                        {
                            "instruction": "SELL",
                            "quantity": 2,
                            "instrument": {
                                "symbol": "XYZ",
                                "assetType": "EQUITY"
                            }
                        }
                    ]
                }
            ]
        });

        let symbol = InstrumentRequest::Equity {
            symbol: "XYZ".to_string(),
        };

        let child_order_req1 = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Limit)
            .session(Session::Normal)
            .duration(Duration::Day)
            .price(45.97)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 2.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let child_order_req2 = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::StopLimit)
            .session(Session::Normal)
            .duration(Duration::Day)
            .price(37.00)
            .stop_price(37.03)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 2.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let order_req = OrderRequestBuilder::default()
            .order_strategy_type(OrderStrategyType::Oco)
            .child_order_strategies(vec![child_order_req1, child_order_req2])
            .build()
            .unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_one_triggers_a_one_cancels_another() {
        // Conditional Order: One Triggers A One Cancels Another
        // Buy 5 shares of XYZ at a Limit price of $14.97 good for the Day. Once filled, 2 sell orders are immediately sent: Sell 5 shares of XYZ at a Limit price of $15.27 and Sell 5 shares of XYZ with a Stop order where the stop price is $11.27. If one of the sell orders fill, the other order is immediately cancelled. Both Sell orders are Good till Cancel. Also known as a 1st Trigger OCO order.
        let expected = json!({
            "orderStrategyType": "TRIGGER",
            "session": "NORMAL",
            "duration": "DAY",
            "orderType": "LIMIT",
            "price": 14.97,
            "orderLegCollection": [
                {
                    "instruction": "BUY",
                    "quantity": 5,
                    "instrument": {
                        "assetType": "EQUITY",
                        "symbol": "XYZ"
                    }
                }
            ],
            "childOrderStrategies": [
                {
                    "orderStrategyType": "OCO",
                    "childOrderStrategies": [
                        {
                            "orderStrategyType": "SINGLE",
                            "session": "NORMAL",
                            "duration": "GOOD_TILL_CANCEL",
                            "orderType": "LIMIT",
                            "price": 15.27,
                            "orderLegCollection": [
                                {
                                    "instruction": "SELL",
                                    "quantity": 5,
                                    "instrument": {
                                        "assetType": "EQUITY",
                                        "symbol": "XYZ"
                                    }
                                }
                            ]
                        },
                        {
                            "orderStrategyType": "SINGLE",
                            "session": "NORMAL",
                            "duration": "GOOD_TILL_CANCEL",
                            "orderType": "STOP",
                            "stopPrice": 11.27,
                            "orderLegCollection": [
                                {
                                    "instruction": "SELL",
                                    "quantity": 5,
                                    "instrument": {
                                        "assetType": "EQUITY",
                                        "symbol": "XYZ"
                                    }
                                }
                            ]
                        }
                    ]
                }
            ]
        });

        let symbol = InstrumentRequest::Equity {
            symbol: "XYZ".to_string(),
        };

        let child_child_order_req1 = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Limit)
            .session(Session::Normal)
            .duration(Duration::GoodTillCancel)
            .price(15.27)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 5.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let child_child_order_req2 = OrderRequestBuilder::default()
            .order_type(OrderTypeRequest::Stop)
            .session(Session::Normal)
            .duration(Duration::GoodTillCancel)
            .stop_price(11.27)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 5.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let child_order_req = OrderRequestBuilder::default()
            .order_strategy_type(OrderStrategyType::Oco)
            .child_order_strategies(vec![child_child_order_req1, child_child_order_req2])
            .build()
            .unwrap();
        let order_req = OrderRequestBuilder::default()
            .order_strategy_type(OrderStrategyType::Trigger)
            .session(Session::Normal)
            .duration(Duration::Day)
            .order_type(OrderTypeRequest::Limit)
            .price(14.97)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Buy,
                quantity: 5.0,
                instrument: symbol.clone(),
            }])
            .child_order_strategies(vec![child_order_req])
            .build()
            .unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }

    #[test]
    fn test_sell_trailing_stop() {
        // Sell Trailing Stop: Stock
        // Sell 10 shares of XYZ with a Trailing Stop where the trail is a -$10 offset from the time the order is submitted. As the stock price goes up, the -$10 trailing offset will follow. If stock XYZ goes from $110 to $130, your trail will automatically be adjusted to $120. If XYZ falls to $120 or below, a Market order is submitted. This order is good for the Day.
        let expected = json!({
            "complexOrderStrategyType": "NONE",
            "orderType": "TRAILING_STOP",
            "session": "NORMAL",
            "stopPriceLinkBasis": "BID",
            "stopPriceLinkType": "VALUE",
            "stopPriceOffset": 10,
            "duration": "DAY",
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [
                {
                    "instruction": "SELL",
                    "quantity": 10,
                    "instrument": {
                        "symbol": "XYZ",
                        "assetType": "EQUITY"
                    }
                }
            ]
        });

        let symbol = InstrumentRequest::Equity {
            symbol: "XYZ".to_string(),
        };

        let order_req = OrderRequestBuilder::default()
            .complex_order_strategy_type(ComplexOrderStrategyType::None)
            .order_type(OrderTypeRequest::TrailingStop)
            .session(Session::Normal)
            .duration(Duration::Day)
            .stop_price_link_basis(StopPriceLinkBasis::Bid)
            .stop_price_link_type(StopPriceLinkType::Value)
            .stop_price_offset(10.0)
            .price(14.97)
            .order_leg_collection(vec![OrderLegCollectionRequest {
                instruction: Instruction::Sell,
                quantity: 10.0,
                instrument: symbol.clone(),
            }])
            .build()
            .unwrap();
        let order_req = serde_json::to_value(order_req).unwrap();
        assert_json_matches!(
            order_req,
            expected,
            Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat)
        );
    }
}
