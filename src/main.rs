use curl::easy::Easy;
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

    let result: bool = check_url(url.as_str());
    if result {
        println!("URL is valid");
    } else {
        println!("URL is invalid");
        return;
    }

    // read the wordlist
    let words = std::fs::read_to_string(wordlist).unwrap();

    // split the wordlist into a vector
    let words: Vec<&str> = words.split_whitespace().collect();

    println!("{} words loaded", words.len());

    // loop through the words and check if the url is valid
    for word in words {
        let new_url = format!("{}/{}", url, word);
        let result: bool = check_url(new_url.as_str());
        if result {
            println!("{} 200 OK", new_url);
        } else {
            println!("{} 404 Not Found", new_url);
        }
    }

    println!("Done");
}

// function to check if the url is valid it returns a 200 status code
fn check_url(url: &str) -> bool {
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.perform().unwrap();
    let code = easy.response_code().unwrap();
    if code == 200 {
        return true;
    }
    return false;
}
