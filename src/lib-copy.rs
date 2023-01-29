use std::fs;
use std::time::Instant;
//use std::env;
use std::error::Error;
use dashmap::DashMap;
use std::net::Ipv4Addr;
use amadeus_streaming::CountMinSketch;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};
//use std::hash::{BuildHasherDefault, Hasher, Hash};

#[derive(Debug)]
pub enum DsType { DASH, CMS, FPDASH }

#[derive(Debug,PartialEq)]
pub enum TimeType { READTIME, WRITETIME, RWTIME }

pub struct Config {
    pub file_path: String,
    pub ds_type: DsType,
    pub time_type : TimeType,
    pub error: f64,
    pub confidence: f64,
    pub max_size: usize,
    pub fp_size: u8,
    pub verbose : bool
}

fn get_errflag (
        args: &mut impl Iterator<Item = String>, error: &mut f64
    ) -> Result<(), &'static str> {
    let errstr = args.next().expect("Expecting an error argument after -err flag");
    if let Ok(err) = errstr.parse::<f64>() {
        *error = err;
    } else {
        return Err("The error argument after -err flag must be a float");
    }
    Ok(())
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let _cmd = args.next();
        let mut time_type = TimeType::WRITETIME;
        let mut error = 0.01;
        let mut confidence = 0.01;
        let mut max_size = 10000;
        let mut fp_size = 8;
        let mut verbose = false;
        let timeflag = String::from("-time");
        let errflag = String::from("-err");
        let confflag = String::from("-conf");
        let sizeflag = String::from("-size");
        let fpflag = String::from("-finger");
        let verbflag = String::from("-v");

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get a file path"),
        };

        let ds_type = match args.next() {
            Some(ds_type) if ds_type == "DASH" => DsType::DASH,
            Some(ds_type) if ds_type == "CMS" => DsType::CMS,
            Some(ds_type) if ds_type == "FPDASH" => DsType::FPDASH,
            Some(_) => return Err("Didn't get a valid data-structure type (DASH or CMS)"),
            None => return Err("Didn't get any data-structure type"),
        };

        while let Some(flag) = args.next() {
            if flag == timeflag {
                time_type = match args.next() {
                    Some(time_type) if time_type == "READ" => TimeType::READTIME,
                    Some(time_type) if time_type == "WRITE" => TimeType::WRITETIME,
                    Some(time_type) if time_type == "RW" => TimeType::RWTIME,
                    Some(_) => return Err("Didn't get a valid measurement type (READ, WRITE, RW)"),
                    None => return Err("Didn't get any measurement type"),
                };
            }
            if flag == errflag {
                //if let Err(res) = get_errflag(args,&mut error) {
                //    return Err(res);
                //}
                let errstr = args.next().expect("Expecting an error argument after -err flag");
                if let Ok(err) = errstr.parse() {
                    error = err;
                } else {
                    return Err("The error argument after -err flag must be a float");
                }
            } else if flag == confflag {
                let confstr = args.next().expect("Expecting a confidence argument after -conf flag");
                if let Ok(conf) = confstr.parse() {
                    confidence = conf;
                } else {
                    return Err("The confidence argument after -conf flag must be a float");
                }
            } else if flag == sizeflag {
                let sizestr = args.next().expect("Expecting a size argument after -size flag");
                if let Ok(size) = sizestr.parse() {
                    max_size = size;
                } else {
                    return Err("The size argument after -size flag must be a positive integer");
                }
            } else if flag == fpflag {
                let fingerstr = args.next().expect("Expecting a fingerprint argument after -finger flag");
                if let Ok(size) = fingerstr.parse() {
                    if size <= 32 {
                        fp_size = size;
                    }
                    else {
                        return Err("The maximal value of fingerprint is 32");
                    }
                } else {
                    return Err("The size argument after -finger flag must be a positive integer");
                }
            } else if flag == verbflag {
                verbose = true;
            } else {
               return Err("Unrecognized argument");
            }
        }
        Ok(Config {
            file_path,
            ds_type,
            time_type,
            error,
            confidence,
            max_size,
            fp_size,
            verbose,
        })
    }
}

#[derive(Hash,PartialEq,Eq,Debug)]
struct FlowId {
    srcip : Ipv4Addr,
    dstip : Ipv4Addr,
}

fn id_from_line(line: String) -> Result<FlowId, Box<dyn Error>> {
    //println!("LINE: {0}", line);
    let mut parts = line.split_whitespace();
    let srcip: Ipv4Addr = Ipv4Addr::new(
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0)
    );
    //println!("SRCIP: {0}", srcip);
    let dstip: Ipv4Addr = Ipv4Addr::new(
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0),
        parts.next().unwrap().parse().unwrap_or_else(|_| 0)
    );
    //println!("DSTIP: {0}", dstip);
    Ok(FlowId { srcip, dstip })
}

fn dash_run(config: Config, processed: Vec<FlowId>) -> Instant {
    let counts = DashMap::new();
    let mut start = Instant::now();
    for id in &processed {
        if let Some(mut count) = counts.get_mut(&id) {
            *count+=1;
        } else {
            counts.insert(id,1);
        }
        if config.time_type == TimeType::RWTIME {
            counts.get_mut(&id);
        }
    }
    if config.time_type == TimeType::READTIME {
        start = Instant::now();
        for id in &processed {
            counts.get_mut(&id);
        }
    }
    if config.verbose {
        println!("COUNTS are {:#?}", counts);
    }
    return start;
}

fn fpdash_run(config: Config, processed: Vec<FlowId>) -> Instant {
    let num : usize = 2_usize.pow(config.fp_size.into());
    let counts = DashMap::with_capacity(num);
    let mut start = Instant::now();
    for id in &processed {
        // id.hash(&mut s);
        //let id : s.finish();
        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        let id = s.finish();
        if let Some(mut count) = counts.get_mut(&id) {
            *count+=1;
        } else {
            counts.insert(id,1);
        }
        if config.time_type == TimeType::RWTIME {
            counts.get_mut(&id);
        }
    }
    if config.time_type == TimeType::READTIME {
        start = Instant::now();
        for id in &processed {
            let mut s = DefaultHasher::new();
            id.hash(&mut s);
            let id = s.finish();
            counts.get_mut(&id);
        }
    }
    if config.verbose {
        println!("COUNTS are {:#?}", counts);
    }
    return start;
}

fn cms_run(config: Config, processed: Vec<FlowId>) -> Instant {
    let mut counts: CountMinSketch<FlowId,u32> = amadeus_streaming::CountMinSketch::new(config.confidence, config.error, ());
    let mut start = Instant::now();
    for id in &processed {
        counts.push(id,&1);
        if config.time_type == TimeType::RWTIME {
            counts.get(id);
        }
    }
    if config.time_type == TimeType::READTIME {
        start = Instant::now();
        for id in &processed {
            counts.get(id);
        }
    }
    if config.verbose {
        println!("COUNTS are {:#?}", counts);
    }
    return start;
}

fn preprocess_contents(contents: String) -> Vec<FlowId> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if let Ok(id) = id_from_line(line.to_string()) {
            result.push(id);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:#?} {:#?} for FILE: {}", config.ds_type, config.time_type, config.file_path);
    let contents = fs::read_to_string(config.file_path.clone())?;
    //if config.verbose {
        println!("PREPROCESSING DONE");
    //}
    let processed = preprocess_contents(contents);
    //let now = Instant::now();
    let start = match config.ds_type {
        DsType::DASH => dash_run(config, processed),
        DsType::CMS => cms_run(config, processed),
        DsType::FPDASH => fpdash_run(config, processed),
        //_ => (),
    };
    let elapsed_time = start.elapsed();
    println!("Running time in microsecs = {}", elapsed_time.as_micros());
    Ok(())
}

//pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//    contents
//        .lines()
//        .filter(|line| line.contains(query))
//        .collect()
//}

//pub fn search_case_insensitive<'a>(
//    query: &str,
//    contents: &'a str,
//) -> Vec<&'a str> {
//    let query = query.to_lowercase();
//    contents
//        .lines()
//        .filter(|line| line.to_lowercase().contains(&query))
//        .collect()
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn one_result() {
//        let query = "duct";
//        let contents = "\
//Rust:
//safe, fast, productive.
//Pick three.";
//
//        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//    }
//
//    #[test]
//    fn case_insensitive() {
//        let query = "rUsT";
//        let contents = "\
//Rust:
//safe, fast, productive.
//Pick three.
//Trust me.";
//
//       assert_eq!(
//            vec!["Rust:", "Trust me."],
//            search_case_insensitive(query, contents)
//        );
//    }
//}