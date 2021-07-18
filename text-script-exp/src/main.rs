use anyhow::{anyhow, Context, Result};
use core::iter::Peekable;
use std::env::{self, Args};
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::mem;
use std::process;
use std::str::Lines;
use tabwriter::TabWriter;

fn main() {
    let filename = parse_args(&mut env::args());
    let contents = read_input(&filename).unwrap_or_else(|err| {
        eprintln!("Could not read input: {}", err);
        process::exit(1);
    });

    print!(
        "{}",
        summarize(&contents).unwrap_or_else(|err| {
            eprintln!("Failed processing contents: {}", err);
            process::exit(1);
        })
    );
}

fn parse_args(args: &mut Args) -> String {
    let mut filename = String::from("-");
    if let Some(f) = args.nth(1) {
        filename = f;
    };
    filename
}

fn read_input(filename: &str) -> Result<String> {
    let stdin; // for holding a reference throughout the scope
    let mut input: Box<dyn Read> =
        if filename == "-" {
            stdin = io::stdin();
            Box::new(stdin.lock())
        } else {
            Box::new(File::open(filename).with_context(|| {
                format!("Could not open file {:?}", &filename)
            })?)
        };

    let mut s = String::new();
    input.read_to_string(&mut s)?;
    Ok(s)
}

/// Parse kubectl-style memory size string in MiB
/// Future improvements: support for other units, proper error handling.
fn parse_mem(s: &str) -> u64 {
    s.trim_end_matches("Mi").parse().unwrap_or_default()
}

/// Per-installation counters and resource usage totals
#[derive(Debug)]
struct InstallationCounters {
    /// name of this installation
    name: String,
    /// tag for this set of counters, e.g. old/new, used to designate which
    /// instance of Prometheus the data is from
    tag: String,
    /// number of shards in this installation
    shard_count: u32,
    /// sum of memory requests from all shards
    mem_requests_total: u64,
    /// sum of memory limits from all shards
    mem_limits_total: u64,
    /// sum of memory utilisation from all shards
    mem_util_total: u64,
}

impl InstallationCounters {
    fn new() -> Self {
        InstallationCounters {
            name: String::new(),
            tag: String::from("new"),
            shard_count: 0,
            mem_requests_total: 0,
            mem_limits_total: 0,
            mem_util_total: 0,
        }
    }
}

/// Implements iteration over utilisation data
struct UtilisationData<'a> {
    data: Peekable<Lines<'a>>,
    current: InstallationCounters,
}

impl<'a> UtilisationData<'a> {
    /// Construct UtilisationData from a str
    fn from(s: &'a str) -> Self {
        UtilisationData {
            data: s.lines().peekable(),
            current: InstallationCounters::new(),
        }
    }
}

impl<'a> Iterator for UtilisationData<'a> {
    type Item = InstallationCounters;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.peek() {
            // first line in the section should contain the installation name
            // but we could be re-entering `next()` after returning a row for
            // an old installation, so check that
            Some(nxt) if nxt.starts_with("# ") => {
                if !self.current.name.is_empty() {
                    // edge case, last iteration returned early (e.g. old
                    // installation row) but it was also the last line in
                    // current section; flush it and start fresh
                    return Some(mem::replace(
                        &mut self.current,
                        InstallationCounters::new(),
                    ));
                }
                let line = self.data.next().unwrap();
                self.current.name.push_str(line.trim_start_matches("# "));
            }
            // non header line, will be processed below
            Some(_) => (),
            // end of input
            None => return None,
        }

        while let Some(line) = self.data.next() {
            if line.trim().is_empty() {
                continue;
            }

            let s: Vec<_> = line.split_whitespace().collect();
            if s.len() < 15 {
                // should not happen but if it does it's not a useful line
                continue;
            }

            let namespace = s[1];

            if namespace == "monitoring" {
                // add data from current line into a new instance of
                // InstallationCounters and return it; we'll continue with this
                // installation's shards on next call to `next()`
                return Some(InstallationCounters {
                    name: self.current.name.clone(),
                    tag: "old".to_owned(),
                    shard_count: 1,
                    mem_requests_total: parse_mem(s[9]),
                    mem_limits_total: parse_mem(s[11]),
                    mem_util_total: parse_mem(s[13]),
                });
            }

            if namespace.ends_with("-prometheus") {
                self.current.shard_count += 1;
                self.current.mem_requests_total += parse_mem(s[9]);
                self.current.mem_limits_total += parse_mem(s[11]);
                self.current.mem_util_total += parse_mem(s[13]);
            }

            if let Some(nxt) = self.data.peek() {
                if nxt.starts_with("# ") {
                    // we reached the end of current section, break out
                    break;
                }
            }
        }

        // end of section or end of input reached, return current counters and
        // reset to prepare for the next section (if any)
        Some(mem::replace(&mut self.current, InstallationCounters::new()))
    }
}

fn summarize(contents: &str) -> Result<String> {
    let mut tw = TabWriter::new(vec![]);

    writeln!(tw, "INSTALLATION\tPROM\tSHARDS\tREQUESTS\tLIMITS\tUTIL")?;
    for row in UtilisationData::from(contents) {
        let line = vec![row.name, row.tag, row.shard_count.to_string()]
            .into_iter()
            .chain(
                vec![
                    row.mem_requests_total,
                    row.mem_limits_total,
                    row.mem_util_total,
                ]
                .into_iter()
                .map(|v| format!("{}Mi", v)),
            )
            .reduce(|a, b| format!("{}\t{}", a, b))
            .ok_or(anyhow!("empty iterator"))?;

        writeln!(tw, "{}", line)?;
    }
    tw.flush()?;

    String::from_utf8(tw.into_inner()?).context("Formatting output failed")
}
