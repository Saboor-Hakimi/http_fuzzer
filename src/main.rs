// loades user arguments so import it
// import request to make requests

use reqwest::Error;
use std::env;

fn main() {
    // if the -u and -w flags are not set, print the help message
    if !env::args().any(|x| x == "-u") || !env::args().any(|x| x == "-w") {
        println!("Usage: fuzzer -u <url> -w <wordlist>");
        return;
    }

    // get the url and wordlist from the arguments
    let url = env::args()
        .nth(env::args().position(|x| x == "-u").unwrap() + 1)
        .unwrap();

    let wordlist = env::args()
        .nth(env::args().position(|x| x == "-w").unwrap() + 1)
        .unwrap();

    // print the url and wordlist
    println!("URL: {}", url);
    println!("Wordlist: {}", wordlist);
}

// function to check if the url is valid it returns a 200 status code
async fn check_url(url: &str) -> bool {
    // create a new request
    let response = reqwest::get("https://api.example.com/data")
        .await?
        .text()
        .await?;

    // check if the status code is 200
    if response.status() == 200 {
        return true;
    }

    return false

}
