mod services;
mod views;

use crate::cli_parser::Provider;

pub fn provider_list_handler() {
    // println!("List of all available weather providers:\n"); # FIXME delete after
    for provider in Provider::get_all_variants() {
        println!(" {}", provider);
    }
}
