#![forbid(unsafe_code)]
#![warn(
clippy::pedantic,
missing_copy_implementations,
missing_debug_implementations,
//missing_docs,
rustdoc::broken_intra_doc_links,
trivial_numeric_casts,
unused_allocation
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::implicit_hasher,
    clippy::similar_names,
    clippy::module_name_repetitions
)]

mod error;
mod servicer;
