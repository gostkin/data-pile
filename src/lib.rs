#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod appender;
mod database;
mod error;
mod flatfile;
mod record;
mod seqno;
pub mod serialization;
#[cfg(test)]
mod testutils;

use appender::Appender;
pub use database::{Database, DatabaseBuilder};
pub use error::Error;
pub use record::Record;
pub use serialization::RecordSerializer;
