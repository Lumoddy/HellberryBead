
mod regex;
use regex::*;

#[proc_macro_derive(Matchine, attributes(matchine, pattern))]
pub fn matchine(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    todo!()
}