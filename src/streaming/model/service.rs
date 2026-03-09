use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Service {
    // ── Admin ─────────────────────────────────────────────────
    #[serde(rename = "ADMIN")]
    Admin,

    // ── Account activity ──────────────────────────────────────
    #[serde(rename = "ACCT_ACTIVITY")]
    AccountActivity,

    // ── Most actives ──────────────────────────────────────────
    #[serde(rename = "ACTIVES_NASDAQ")]
    ActivesNasdaq,
    #[serde(rename = "ACTIVES_NYSE")]
    ActivesNyse,
    #[serde(rename = "ACTIVES_OTCBB")]
    ActivesOtcbb,
    #[serde(rename = "ACTIVES_OPTIONS")]
    ActivesOptions,

    // ── Chart candles ─────────────────────────────────────────
    #[serde(rename = "CHART_EQUITY")]
    ChartEquity,
    #[serde(rename = "CHART_FUTURES")]
    ChartFutures,
    #[serde(rename = "CHART_OPTIONS")]
    ChartOptions,
    #[serde(rename = "CHART_HISTORY_FUTURES")]
    ChartHistoryFutures,

    // ── Level one quotes ──────────────────────────────────────
    #[serde(rename = "QUOTE")]
    Quote,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "LEVELONE_FUTURES")]
    LevelOneFutures,
    #[serde(rename = "LEVELONE_FOREX")]
    LevelOneForex,
    #[serde(rename = "LEVELONE_FUTURES_OPTIONS")]
    LevelOneFuturesOptions,

    // ── Level two order books ─────────────────────────────────
    #[serde(rename = "LISTED_BOOK")]
    ListedBook,
    #[serde(rename = "NASDAQ_BOOK")]
    NasdaqBook,
    #[serde(rename = "OPTIONS_BOOK")]
    OptionsBook,
    #[serde(rename = "FUTURES_BOOK")]
    FuturesBook,
    #[serde(rename = "FOREX_BOOK")]
    ForexBook,
    #[serde(rename = "FUTURES_OPTIONS_BOOK")]
    FuturesOptionsBook,

    // ── News ──────────────────────────────────────────────────
    #[serde(rename = "NEWS_HEADLINE")]
    NewsHeadline,
    #[serde(rename = "NEWS_HEADLINELIST")]
    NewsHeadlineList,
    #[serde(rename = "NEWS_STORY")]
    NewsStory,

    // ── Time & sale ───────────────────────────────────────────
    #[serde(rename = "TIMESALE_EQUITY")]
    TimesaleEquity,
    #[serde(rename = "TIMESALE_FUTURES")]
    TimesaleFutures,
    #[serde(rename = "TIMESALE_FOREX")]
    TimesaleForex,
    #[serde(rename = "TIMESALE_OPTIONS")]
    TimesaleOptions,

    // ── Misc ──────────────────────────────────────────────────
    #[serde(rename = "STREAMER_SERVER")]
    StreamerServer,
}

impl Service {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "ADMIN",
            Self::AccountActivity => "ACCT_ACTIVITY",
            Self::ActivesNasdaq => "ACTIVES_NASDAQ",
            Self::ActivesNyse => "ACTIVES_NYSE",
            Self::ActivesOtcbb => "ACTIVES_OTCBB",
            Self::ActivesOptions => "ACTIVES_OPTIONS",
            Self::ChartEquity => "CHART_EQUITY",
            Self::ChartFutures => "CHART_FUTURES",
            Self::ChartOptions => "CHART_OPTIONS",
            Self::ChartHistoryFutures => "CHART_HISTORY_FUTURES",
            Self::Quote => "QUOTE",
            Self::Option => "OPTION",
            Self::LevelOneFutures => "LEVELONE_FUTURES",
            Self::LevelOneForex => "LEVELONE_FOREX",
            Self::LevelOneFuturesOptions => "LEVELONE_FUTURES_OPTIONS",
            Self::ListedBook => "LISTED_BOOK",
            Self::NasdaqBook => "NASDAQ_BOOK",
            Self::OptionsBook => "OPTIONS_BOOK",
            Self::FuturesBook => "FUTURES_BOOK",
            Self::ForexBook => "FOREX_BOOK",
            Self::FuturesOptionsBook => "FUTURES_OPTIONS_BOOK",
            Self::NewsHeadline => "NEWS_HEADLINE",
            Self::NewsHeadlineList => "NEWS_HEADLINELIST",
            Self::NewsStory => "NEWS_STORY",
            Self::TimesaleEquity => "TIMESALE_EQUITY",
            Self::TimesaleFutures => "TIMESALE_FUTURES",
            Self::TimesaleForex => "TIMESALE_FOREX",
            Self::TimesaleOptions => "TIMESALE_OPTIONS",
            Self::StreamerServer => "STREAMER_SERVER",
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
