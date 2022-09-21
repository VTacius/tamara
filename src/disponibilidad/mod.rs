mod icmp;
mod http;
mod db;

pub use self::icmp::PinnerFuture;
pub use self::http::{http_future, VeredictoHTTP};
pub use self::db::{db_future, VeredictoDB};