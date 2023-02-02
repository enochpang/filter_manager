#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, LineWriter, Write};

use filter_manager::lexer::Lexer;
use filter_manager::parser::{Parser, RuleItem};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let mut buffered = BufReader::new(input);

    let mut data = Vec::new();
    buffered.read_until(b'\0', &mut data)?;

    let mut lexer = Lexer::new(data);
    let mut parser = Parser::new(&mut lexer);
    let rules = parser.parse().unwrap();

    show_dest_count(rules);

    Ok(())
}

fn show_dest_count(rules: Vec<RuleItem>) {
    let mut res: HashMap<String, i32> = HashMap::new();

    for rule in rules {
        match rule {
            RuleItem::Setting(_) => (),
            RuleItem::Filter(fr) => {
                if fr.destination == "*" {
                    continue;
                }

                res.entry(fr.destination.to_string())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }

    let mut items = Vec::from_iter(res);
    items.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    for (key, val) in items {
        if val > 1 {
            println!("{} -> {}", key, val)
        }
    }
}

fn show_subdomains(rules: Vec<RuleItem>) {
    for rule in rules {
        match rule {
            RuleItem::Setting(_) => {}
            RuleItem::Filter(fr) => {
                if let Some(host) = fr.source.host() {
                    let s = match psl::domain_str(host) {
                        Some(x) => x,
                        None => host,
                    };

                    if s.len() < host.len() {
                        println!("{} {}", host, fr.destination)
                    }
                }
            }
        }
    }
}

fn write_without_subdomain(rules: Vec<RuleItem>) -> Result<(), Error> {
    let outfile = File::create("output.txt")?;
    let mut outfile = LineWriter::new(outfile);

    for rule in rules {
        match rule {
            RuleItem::Setting(sr) => {
                outfile.write_fmt(format_args!("{} {} {} \n", sr.name, sr.location, sr.val))?;
            }
            RuleItem::Filter(fr) => {
                if let Some(host) = fr.source.host() {
                    let s = match psl::domain_str(host) {
                        Some(x) => x,
                        None => host,
                    };

                    outfile.write_fmt(format_args!(
                        "{} {} {} {}\n",
                        s, fr.destination, fr.req_type, fr.action_type,
                    ))?;
                } else {
                    outfile.write_fmt(format_args!(
                        "{} {} {} {}\n",
                        fr.source, fr.destination, fr.req_type, fr.action_type,
                    ))?;
                }
            }
        }
    }

    outfile.flush()?;

    Ok(())
}
