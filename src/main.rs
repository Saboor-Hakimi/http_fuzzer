// loades user arguments so import it
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
