//! # About
//! A high performance interface for the Roblox API.
//!
//! This library is designed to be high-performance capable, meaning
//! that a [`Client`] is designed to work with proxies, as well as make
//! multiple requests in parallel. All API calls are made through a [`Client`].
//!
//! Extensive documentation is used throughout this crate.
//! All public methods in this crate are documented and have at least one corresponding example.
//!
//! # Coverage
//! * Auth API
//!   - Force Refresh Xcsrf - [`Client::force_refresh_xcsrf`]
//! * BEDEV2 API
//!   - Fetch Non-Tradable Limited Details - [`Client::non_tradable_limited_details`]
//!   - Fetch Collectible Product ID - [`Client::collectible_product_id`]
//!   - Fetch Collectible Product ID Bulk - [`Client::collectible_product_id_bulk`]
//!   - Fetch Collectible Creator ID - [`Client::collectible_creator_id`]
//!   - Purchase Non-Tradable Limited - [`Client::purchase_non_tradable_limited`]
//! * Catalog API
//!   - Fetch Item Details - [`Client::item_details`]
//!   - Fetch Product ID - [`Client::product_id`]  
//!   - Fetch Product ID Bulk - [`Client::product_id_bulk`]
//!   - Fetch Collectible Item ID - [`Client::collectible_item_id`]
//!   - Fetch Collectible Item ID Bulk - [`Client::collectible_item_id_bulk`]
//!   - Avatar Catalog Search - [`Client::avatar_catalog_search`]
//! * Chat API
//!   - Fetch Unread Conversation Count - [`Client::unread_conversation_count`]
//! * Economy API
//!   - Fetch Robux Balance - [`Client::robux`]
//!   - Fetch Resellers - [`Client::resellers`]
//!   - Fetch User Sales - [`Client::user_sales`]
//!   - Put Limited On Sale - [`Client::put_limited_on_sale`]
//!   - Take Limited Off Sale - [`Client::take_limited_off_sale`]
//!   - Purchase Tradable Limited - [`Client::purchase_tradable_limited`]
//! * Group API
//!   - Fetch Group Roles - [`Client::group_roles`]
//!   - Fetch Group Role Members - [`Client::group_role_members`]
//!   - Set Group Member Role - [`Client::set_group_member_role`]
//! * Presence API
//!   - Register Presence - [`Client::register_presence`]
//! * Private Messages API
//!  - Fetch Messages - [`Client::messages`]
//! * Trades API
//!   - Fetch Trades List - [`Client::trades`]
//! * Users API
//!   - Fetch User ID - [`Client::user_id`]
//!   - Fetch Username - [`Client::username`]
//!   - Fetch Display Name - [`Client::display_name`]
//!   - User Search - [`Client::user_search`]
//!   - Fetch User Details - [`Client::user_details`]
//!
//! # Quick Start Examples
//!
//! ## Example 1 - Purchase Free UGC Limited
//! This code snippet allows you to purchase a free ugc limited.
//!
//! It can be modified to purchase a non-free ugc limited by changing the price.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//! // Replace this value with the item id of the item you want to purchase.
//! const ITEM_ID: u64 = 13119979433;
//! // Replace this value if you want to purchase a non-free item.
//! const PRICE: u64 = 0;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let collectible_item_id = client.collectible_item_id(ITEM_ID).await?;
//!
//!     let collectible_product_id = client
//!         .collectible_product_id(collectible_item_id.clone())
//!         .await?;
//!
//!     let collectible_creator_id = client
//!         .collectible_creator_id(collectible_item_id.clone())
//!         .await?;
//!
//!     client
//!         .purchase_non_tradable_limited(
//!             collectible_item_id,
//!             collectible_product_id,
//!             collectible_creator_id,
//!             PRICE,
//!         )
//!         .await?;
//!
//!     println!("Purchased item {} for {} robux!", ITEM_ID, PRICE);
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 2 - Fetch User Info
//!
//! This code snippet allows you to get your current robux, id, username, and display name.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let robux = client.robux().await?;
//!     let user_id = client.user_id().await?;
//!     let username = client.username().await?;
//!     let display_name = client.display_name().await?;    
//!
//!     println!("Robux: {}", robux);
//!     println!("User ID: {}", user_id);
//!     println!("Username: {}", username);
//!     println!("Display Name: {}", display_name);
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 3 - Fetch Price of Tradable Limited
//!
//! This code snippet allows you to view the lowest price of a tradable limited item by
//! fetching a list of reseller listings.
//!
//! ```no_run
//! // Replace this value with your own roblosecurity token.
//! const ROBLOSECURITY: &str = "your-roblosecurity-token";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new()
//!         .roblosecurity(ROBLOSECURITY.to_string())
//!         .build();
//!
//!     let item_id = 1365767;
//!     let limit = roboat::Limit::Ten;
//!     let cursor = None;
//!
//!     let (resellers, _) = client.resellers(item_id, limit, cursor).await?;
//!
//!     println!("Lowest Price for Valkyrie Helm: {}", resellers[0].price);  
//!
//!     Ok(())   
//! }
//! ```
//!
//! ## Example 4 - Fetch Item Details
//!
//! This code snippet allows you to get the details of an item.
//!
//! ```no_run
//! use roboat::catalog::{Item, ItemType};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = roboat::ClientBuilder::new().build();
//!
//!     let item = Item {
//!         item_type: ItemType::Asset,
//!         id: 1365767,
//!     };
//!
//!     let details = &client.item_details(vec![item]).await?[0];
//!
//!     let name = &details.name;
//!     let description = &details.description;
//!     let creator_name = &details.creator_name;
//!     let price = details.price.unwrap_or(0);
//!
//!     println!("Name: {}", name);
//!     println!("Description: {}", description);
//!     println!("Creator Name: {}", creator_name);
//!     println!("Price: {}", price);
//!
//!     Ok(())   
//! }
//! ```

#![warn(missing_docs)]

// Re-export reqwest so people can use the correct version.
pub use reqwest;

pub use bedev2::PurchaseNonTradableLimitedError;
pub use client::{Client, ClientBuilder};
pub use economy::PurchaseTradableLimitedError;

/// A module for endpoints prefixed with <https://auth.roblox.com/*>.
mod auth;
/// A module for endpoints prefixed with <https://apis.roblox.com/*>.
pub mod bedev2;
/// A module for endpoints prefixed with <https://catalog.roblox.com/*>.
pub mod catalog;
/// A module for endpoints prefixed with <https://chat.roblox.com/*>.
mod chat;
/// A module related to the [`Client`] struct.
mod client;
/// A module for endpoints prefixed with <https://economy.roblox.com/*>.
pub mod economy;
/// A module for endpoints prefixed with <https://groups.roblox.com/*>.
pub mod groups;
/// A module for endpoints prefixed with <https://presence.roblox.com/*>.
mod presence;
/// A module for endpoints prefixed with <https://privatemessages.roblox.com/*>.
pub mod private_messages;
/// A module for endpoints prefixed with <https://trades.roblox.com/*>.
pub mod trades;
/// A module for endpoints prefixed with <https://users.roblox.com/*>.
pub mod users;
/// A module related to validating requests.
mod validation;

// todo: endpoints that require premium/robux to test: recent trades, send trade, buy limited item, buy non-limited item
// todo: inventory api, groups api, follow api
// todo: add usage to readme
// todo: every type should have an explanation of the typical means by which the user will construct or fetch it, if the answer isn't “this is a struct literal with public methods”.
// todo: figure out authtickets
// todo: hide reqwest types
// todo: maybe respect cookies returned
// todo: maybe post on devforums, reddit, maybe the rust server
// todo: put string of parsing error in MalformedResponse
// todo: apparently a v2 details api does 500 items at once
// todo: name apis api bedev2 or something
// todo: rename methods and docs to remain consistent over what non-tradable limiteds are called.
// todo: add method to get items from catalog
// todo: make ItemDetails include both price and lowest price
// todo: replace urls with the form GROUP_ROLES_API.replace("{group_id}", &group_id.to_string());
// todo: maybe add stronger types for stuff like cursors? stuff that can be returned basically
// todo: add doc example and example count somewhere
// todo: add message apis

use serde::{Deserialize, Serialize};

// Used in request header keys.
const XCSRF_HEADER: &str = "x-csrf-token";
// The user agent used for fussy endpoints.
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:101.0) Gecko/20100101 Firefox/101.0";
// The content type used for fussy endpoints.
const CONTENT_TYPE: &str = "application/json;charset=utf-8";

/// The maximum amount of instances to return from an endpoint. Used as a parameter in various methods that call
/// endpoints. This is an enum instead of an integer as these are the only values that are accepted by Roblox
/// for the limit parameter.
#[allow(missing_docs)]
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize, Copy,
)]
pub enum Limit {
    #[default]
    Ten,
    TwentyFive,
    Fifty,
    Hundred,
}

impl Limit {
    fn to_u64(self) -> u64 {
        match self {
            Limit::Ten => 10,
            Limit::TwentyFive => 25,
            Limit::Fifty => 50,
            Limit::Hundred => 100,
        }
    }
}

/// The universal error used in this crate. Encapsulates any sub-errors used in this crate.
#[non_exhaustive]
#[derive(thiserror::Error, Debug, Default)]
pub enum RoboatError {
    /// Used when an endpoint returns status code 429.
    #[default]
    #[error("Too Many Requests")]
    TooManyRequests,
    /// Used when an endpoint returns status code 500.
    #[error("Internal Server Error")]
    InternalServerError,
    /// Used when an endpoint returns status code 400 and does not embed an error.
    /// This is used when the server cannot process the data sent, whether
    /// it be because it is in the wrong format or it contains too much data.
    #[error("Bad Request")]
    BadRequest,
    /// Returned when the user does not have a valid roblosecurity, or
    /// does not have authorization to access the endpoint.
    ///
    /// This is also used as the backup error when an endpoint returns a 401 status code
    /// but the error cannot be parsed from the response.
    ///
    /// Roblox error code 0.
    #[error("Invalid Roblosecurity")]
    InvalidRoblosecurity,
    /// Returned when the endpoint returns a 401 status code, but the error response
    /// contains an unknown Roblox error code.
    #[error("Unknown Roblox Error Code {code}: {message}")]
    UnknownRobloxErrorCode {
        /// The error code (not status code) returned by roblox.
        code: u16,
        /// The error message returned by roblox.
        message: String,
    },
    /// Used when no roblosecurity is set, on an endpoint that requires it.
    #[error("Roblosecurity Not Set")]
    RoblosecurityNotSet,
    /// Used for any status codes that do not fit any enum variants of this error.
    /// If you encounter this enum variant, please submit an issue so a variant can be
    /// made or the crate can be fixed.
    #[error("Unidentified Status Code {0}")]
    UnidentifiedStatusCode(u16),
    /// Used when the response from an API endpoint is malformed.
    #[error("Malformed Response. If this occurs often it may be a bug. Please report it to the issues page.")]
    MalformedResponse,
    /// Used when an endpoint rejects a request due to an invalid xcsrf.
    /// Mostly used internally invalid xcsrf is returned due to the fact that rust does not
    /// allow async recursion without making a type signature extremely messy.
    #[error("Invalid Xcsrf. New Xcsrf Contained In Error.")]
    InvalidXcsrf(String),
    /// Used when an endpoint returns a 403 status code, but the response does not contain
    /// a new xcsrf.
    #[error("Missing Xcsrf")]
    XcsrfNotReturned,
    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_tradable_limited`].
    #[error("{0}")]
    PurchaseTradableLimitedError(PurchaseTradableLimitedError),
    /// Custom Roblox errors sometimes thrown when the user calls [`Client::purchase_non_tradable_limited`].
    #[error("{0}")]
    PurchaseNonTradableLimitedError(PurchaseNonTradableLimitedError),
    /// Used for any reqwest error that occurs.
    #[error("RequestError {0}")]
    ReqwestError(reqwest::Error),
}

/// The universal struct for a Roblox user in this crate.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}
