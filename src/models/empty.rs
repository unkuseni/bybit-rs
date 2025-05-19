use crate::prelude::*;

/// An empty struct used as a placeholder for API responses where no additional data is returned.
///
/// This is commonly used in Bybit API responses for fields like `ret_ext_info` where the API may return an empty object (`{}`) to maintain a consistent response structure. For trading bots, this struct is typically ignored unless you need to verify the presence of an empty object for error handling or response validation.
#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Empty {}
