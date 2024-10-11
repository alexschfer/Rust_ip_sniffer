use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

// Maximum number of ports to scan (from 0 to 65535)
const MAX: u16 = 65535;

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        match IpAddr::from_str(&f) {
            Ok(ipaddr) => {
                return Ok(Arguments {
                    flag: String::from(""),
                    ipaddr,
                    threads: 4,
                });
            }
            Err(_) => {
                let flag = args[1].clone();
                if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                    println!(
                        "Usage: -j to select how many threads you want
                    \r\n       -h or -help to show this help message"
                    );
                    return Err("help");
                } else if flag.contains("-h") || flag.contains("-help") {
                    return Err("to many arguments");
                } else if flag.contains("-j") && args.len() == 4 {
                    let ipaddr = match IpAddr::from_str(&args[3]) {
                        Ok(s) => s,
                        Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                    };
                    let threads = match args[2].parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => return Err("failed to parse thread number"),
                    };
                    return Ok(Arguments {
                        threads,
                        flag,
                        ipaddr,
                    });
                } else {
                    return Err("invalid syntax");
                }
            }
        }
    }
}

// Function to scan a range of ports on the given IP address using multiple threads
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1; // Start scanning from the next port
    loop {
        // Attempt to connect to the current port
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                // If connection is successful, print a dot and flush the output
                print!(".");
                io::stdout().flush().unwrap();
                // Send the open port back to the main thread through the channel
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        // Stop scanning if we've reached the last port
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads; // Move to the next port in this thread's range
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = match Arguments::new(&args) {
        Ok(args) => args, // If parsing is successful, return the `Arguments`
        Err(err) => {
            match err {
                "help" => {
                    process::exit(0); // If the error is "help", exit gracefully
                }
                _ => {
                    eprintln!("{} problem parsing arguments: {}", program, err); // Print other errors
                    process::exit(0); // Exit with an error code
                }
            }
        }
    };

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    // Spawn threads to scan ports concurrently
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for val in rx {
        out.push(val); // Collect all open ports from the receiver
    }

    println!("");
    out.sort();
    for val in out {
        println!("{} is open", val); // Print the open ports
    }
}
