// declare module
mod forex;

// grant this file access to function get_exchange_rates in forex module
use forex::get_exchange_rates;

// use forex::*; // use * to grant access to everything in forex module

fn main() {
  get_exchange_rates();
  forex::perform_exchange();
}
