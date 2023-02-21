use std::{env};
use std::collections::HashMap;
use std::ffi::OsStr;

use chrono::{Utc};

use headless_chrome::{Browser, LaunchOptionsBuilder};

use num_traits::FromPrimitive;


pub fn main() {
    let options = LaunchOptionsBuilder::default()
        .args(vec![
            &OsStr::new("--disable-gpu")])
        .build()
        .unwrap();
    let browser = Browser::new(options).unwrap();
    let tab = browser.new_tab().unwrap();

    // Navigate to the page
    let response = tab.navigate_to("https://www.winrate.io/volatility-index").unwrap();

    // Wait for the table to be present
    let start1 = Utc::now();
    let table = tab.wait_for_elements_by_xpath("//table[@id='table']").unwrap();
    println!("wait_for_elements_by_xpath took {}ms", (Utc::now().timestamp_millis() - start1.timestamp_millis()));

    for table_elem in table {
        let start2 = Utc::now();
        match table_elem.find_elements_by_xpath("//tbody/tr/td") {
            Ok(elements) => {
                println!("  find_elements_by_xpath took {}ms", (Utc::now().timestamp_millis() - start2.timestamp_millis()));
                for el in elements {
                    let start3 = Utc::now();
                    match el.get_inner_text() {
                        Ok(text) => {
                            println!("text: {}", text);
                        }
                        Err(_) => {}
                    }
                    println!("  get_inner_text took {}ms", (Utc::now().timestamp_millis() - start3.timestamp_millis()));
                }
            }
            Err(err) => { println!("could not find table: {}", err) }
        }
    }
    println!("total scan took {}ms", (Utc::now().timestamp_millis() - start1.timestamp_millis()));
    response.close(true).unwrap();
}
