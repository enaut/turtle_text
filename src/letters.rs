// The implementations for each char works under the assumption that we will
// start the drawing in the left bottom corner on an imaginary rectangule
// heading to top.
// So each individual char impl will end in this default position for the next
// char to start.
mod a_to_d;
mod diacritics;
mod e_to_i;
mod j_to_o;
mod numbers;
mod p_to_t;
mod punctuation;
mod u_to_z;

pub use a_to_d::*;
pub use diacritics::*;
pub use e_to_i::*;
pub use j_to_o::*;
pub use numbers::*;
pub use p_to_t::*;
pub use punctuation::*;
pub use u_to_z::*;
