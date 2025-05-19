include!("../generated/lib.rs");

use dawn_sys as sys;
use std::{
    slice,
    mem,
    ptr,
    ops::Deref,
};

mod instance;
pub use instance::*;



