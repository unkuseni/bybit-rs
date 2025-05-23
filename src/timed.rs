#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Timed<T> {
    /// The time when the data was received (in milliseconds).
    ///
    /// This timestamp indicates when the data was fetched or updated. Bots can use this to synchronize data with other events or to measure latency.
    pub time: u64,
    /// The actual data of type `T`.
    ///
    /// This field contains the specific data structure that holds the relevant information for the bot's operation. Bots can use this to access and manipulate the data as needed.
    pub data: T,
}
