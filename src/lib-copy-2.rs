use std::fs;
use std::time::{Instant, Duration};
//use std::env;
use std::error::Error;
use clap::Parser;
use std::str::FromStr;
use dashmap::DashMap;
use std::net::Ipv4Addr;
use amadeus_streaming::CountMinSketch;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};
//use std::hash::{BuildHasherDefault, Hasher, Hash};

pub mod more_streaming;

use crate::more_streaming::nitro_cms::NitroCMS;
use crate::more_streaming::space_saving::SpaceSaving;
use crate::more_streaming::nitro_hash::NitroHash;
use crate::more_streaming::traits::ItemIncrement;
use crate::more_streaming::traits::ItemQuery;

#[derive(Debug,Clone)]
pub enum DsType { DASH, CMS, NitroCMS, FPDASH, SpaceSaving, NitroHash }

impl FromStr for DsType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DASH" => Ok(DsType::DASH),
            "CMS" => Ok(DsType::CMS),
            "NitroCMS" => Ok(DsType::NitroCMS),
            "FPDASH" => Ok(DsType::FPDASH),
            "SpaceSaving" => Ok(DsType::SpaceSaving),
            "NitroHash" => Ok(DsType::NitroHash),
            _ => Err(format!("Unrecognized DsType {s}: try DASH, CMS, NitroCMS, SpaceSaving, FDDASH, or NitroHash"))
        }
    }
}


#[derive(Debug,PartialEq,Clone)]
pub enum TimeType { READTIME, WRITETIME, RWTIME }

impl FromStr for TimeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "READTIME" => Ok(TimeType::READTIME),
            "WRITETIME" => Ok(TimeType::WRITETIME),
            "RWTIME" => Ok(TimeType::RWTIME),
            _ => Err(format!("Unrecognized TimeType {s}: try READTIME, WRITETIME, or RWTIME"))
        }
    }
}

impl ToString for TimeType {
    fn to_string(&self) -> String {
        match self {
            TimeType::READTIME => "READTIME".to_string(),
            TimeType::WRITETIME => "WRITETIME".to_string(),
            TimeType::RWTIME => "RWTIME".to_string(),
        }
    }
}

#[derive(Parser,Debug)]
#[clap(author="Roy Friedman", version, about="Banchmarking frequency sketches")]
pub struct Config {
    #[clap(short, long)]
    pub file_path: String,
    #[clap(short, long)]
    pub ds_type: DsType,
    #[clap(short, long, default_value_t = TimeType::WRITETIME)]
    pub time_type : TimeType,
    #[clap(short, long, default_value_t = 0.01)]
    pub error: f64,
    #[clap(short, long, default_value_t = 0.01)]
    pub confidence: f64,
    #[clap(short, long, default_value_t = 10000)]
    pub max_size: usize,
    #[clap(long, default_value_t = 8)]
    pub fp_size: u8,
    #[clap(short, long, default_value_t = 0.01)]
    pub sample: f64,
    #[clap(short, long, default_value_t = false)]
    pub verbose : bool,
    #[clap(short, long, default_value_t = false)]
    pub rap : bool,
    #[clap(long, default_value_t = false)]
    pub compare : bool,
}

#[derive(Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct FlowId {
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

fn dash_run(config: Config, processed: Vec<FlowId>) -> Duration {
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
    return start.elapsed();
}

fn fpdash_run(config: Config, processed: Vec<FlowId>) -> Duration {
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
    return start.elapsed();
}

fn nitrocms_accuracy(config: Config, processed: Vec<FlowId>) -> f64 {
    let mut msre = 0.0;
    let baseline = DashMap::new();
    let mut counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, config.sample, ());
    for id in &processed {
        if let Some(mut count) = baseline.get_mut(&id) {
            *count+=1;
        } else {
            baseline.insert(id,1);
        }
        counts.push(id,&1,config.sample);
        if let Some(count) = baseline.get(&id) {
            if config.verbose {
                println!("{:#?} in Baseline {} in CMS {}", id, *count, counts.get(id));
            }
            msre += (f64::from(counts.get(id)) - f64::from(*count)).powi(2);
        }
    }
    println!("Trace length = {}", (&processed).len());
    return msre.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap();
}

fn nitrocms_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let mut counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, config.sample, ());
    let mut start = Instant::now();
    for id in &processed {
        counts.push(id,&1,config.sample);
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
    return start.elapsed();
}

fn cms_accuracy(config: Config, processed: Vec<FlowId>) -> f64 {
    let mut msre = 0.0;
    let baseline = DashMap::new();
    let mut counts: CountMinSketch<FlowId,u32> = amadeus_streaming::CountMinSketch::new(config.confidence, config.error, ());
    for id in &processed {
        if let Some(mut count) = baseline.get_mut(&id) {
            *count+=1;
        } else {
            baseline.insert(id,1);
        }
        counts.push(id,&1);
        if let Some(count) = baseline.get(&id) {
            if config.verbose {
                println!("{:#?} in Baseline {} in CMS {}", id, *count, counts.get(id));
            }
            msre += (f64::from(counts.get(id)) - f64::from(*count)).powi(2);
        }
    }
    println!("Trace length = {}", (&processed).len());
    return msre.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap();
}

fn cms_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let mut counts: CountMinSketch<FlowId,u32> = 
                    amadeus_streaming::CountMinSketch::new(config.confidence, config.error, ());
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
    return start.elapsed();
}

fn space_accuracy(config: Config, processed: Vec<FlowId>) -> f64 {
    let mut msre = 0.0;
    let baseline = DashMap::new();
    let mut counts: SpaceSaving<FlowId,u32> = SpaceSaving::new(config.error, config.rap);
    for id in &processed {
        if let Some(mut count) = baseline.get_mut(&id) {
            *count+=1;
        } else {
            baseline.insert(id,1);
        }
        counts.insert(*id);
        if let Some(count) = baseline.get(&id) {
            if config.verbose {
                println!("{:#?} in Baseline {} in Space {}", id, *count, counts.get(*id));
            }
            msre += (f64::from(counts.get(*id)) - f64::from(*count)).powi(2);
        }
    }
    println!("Trace length = {}", (&processed).len());
    return msre.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap();
}

fn space_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let mut counts: SpaceSaving<FlowId,u32> = SpaceSaving::new(config.error, config.rap);
    return generic_time(config, processed, counts);
    //let mut start = Instant::now();
    //for id in &processed {
    //    counts.insert(*id);
    //    if config.time_type == TimeType::RWTIME {
    //        counts.get(*id);
    //    }
    //}
    //if config.time_type == TimeType::READTIME {
    //    start = Instant::now();
    //    for id in &processed {
    //        counts.get(*id);
    //    }
    //}
    //if config.verbose {
    //    println!("COUNTS are {:#?}", counts);
    //}
    //return start.elapsed();
}

fn nitrohash_accuracy(config: Config, processed: Vec<FlowId>) -> f64 {
    let mut msre = 0.0;
    let baseline = DashMap::new();
    let mut counts: NitroHash<FlowId,u32> = NitroHash::new(config.sample);
    for id in &processed {
        if let Some(mut count) = baseline.get_mut(&id) {
            *count+=1;
        } else {
            baseline.insert(id,1);
        }
        counts.insert(*id);
        if let Some(count) = baseline.get(&id) {
            if config.verbose {
                println!("{:#?} in Baseline {} in NitroHash {}", id, *count, counts.get(*id));
            }
            msre += (f64::from(counts.get(*id)) - f64::from(*count)).powi(2);
        }
    }
    println!("Trace length = {}", (&processed).len());
    return msre.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap();
}

fn nitrohash_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let mut counts: NitroHash<FlowId,u32> = NitroHash::new(config.sample);
    return generic_time(config, processed, counts);
    //let mut start = Instant::now();
    //for id in &processed {
    //    counts.insert(*id);
    //    if config.time_type == TimeType::RWTIME {
    //        counts.get(*id);
    //    }
    //}
    //if config.time_type == TimeType::READTIME {
    //    start = Instant::now();
    //    for id in &processed {
    //        counts.get(*id);
    //    }
    //}
    //if config.verbose {
    //    println!("COUNTS are {:#?}", counts);
    //}
    //return start.elapsed();
}

fn generic_time<Q: Sized>(config: Config, processed: Vec<FlowId>, mut counts: Q) -> Duration
where
Q: ItemIncrement + ItemQuery + std::fmt::Debug,
{
    let mut start = Instant::now();
    for id in &processed {
        counts.item_increment(*id);
        if config.time_type == TimeType::RWTIME {
            counts.item_query(*id);
        }
    }
    if config.time_type == TimeType::READTIME {
        start = Instant::now();
        for id in &processed {
            counts.item_query(*id);
        }
    }
    if config.verbose {
        println!("COUNTS are {:#?}", counts);
    }
    return start.elapsed();
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
    if config.compare {
        let msre  = match config.ds_type {
            DsType::DASH => 0.0,
            DsType::CMS => cms_accuracy(config, processed),
            DsType::NitroCMS => nitrocms_accuracy(config, processed),
            DsType::FPDASH => 0.0,
            DsType::SpaceSaving => space_accuracy(config, processed),
            DsType::NitroHash => nitrohash_accuracy(config, processed),
            //_ => (),
        };
        println!("Calculated MSRE is {}", msre);
    } else {
        let elapsed_time  = match config.ds_type {
            DsType::DASH => dash_run(config, processed),
            DsType::CMS => cms_time(config, processed),
            DsType::NitroCMS => nitrocms_time(config, processed),
            DsType::FPDASH => fpdash_run(config, processed),
            DsType::SpaceSaving => space_time(config, processed),
            DsType::NitroHash => nitrohash_time(config, processed),
            //_ => (),
        };
        println!("Running time in microsecs = {}", elapsed_time.as_micros());
    }
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