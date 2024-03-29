use std::fs;
use std::time::{Instant, Duration};
//use std::env;
use std::error::Error;
use clap::Parser;
use std::str::FromStr;
use std::net::Ipv4Addr;
//use amadeus_streaming::CountMinSketch;
use std::collections::{HashMap,hash_map::DefaultHasher};
use std::hash::{Hasher, Hash};
use num_traits::abs;
//use std::cmp::max;
//use std::mem::size_of;

pub mod more_streaming;

use crate::more_streaming::nitro_cms::NitroCMS;
use crate::more_streaming::space_saving::SpaceSaving;
use crate::more_streaming::nitro_hash::NitroHash;
use crate::more_streaming::cuckoo::CuckooCountingFilter;
use crate::more_streaming::nitro_cuckoo::NitroCuckoo;
use crate::more_streaming::facs::FACS;
use crate::more_streaming::traits::{ItemIncrement,ItemQuery,PrintMemoryInfo};
//use crate::more_streaming::f64_to_usize;

#[cfg(feature = "stats")]
use std::alloc;
#[cfg(feature = "stats")]
use cap::Cap;

#[cfg(feature = "stats")]
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

#[derive(Debug,Clone)]
pub enum DsType { HASH, CMS, NitroCMS, FPDASH, SpaceSaving, NitroHash, Cuckoo, NitroCuckoo, FACS }

impl FromStr for DsType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HASH" => Ok(DsType::HASH),
            "CMS" => Ok(DsType::CMS),
            "NitroCMS" => Ok(DsType::NitroCMS),
            "FPDASH" => Ok(DsType::FPDASH),
            "SpaceSaving" => Ok(DsType::SpaceSaving),
            "NitroHash" => Ok(DsType::NitroHash),
            "Cuckoo" => Ok(DsType::Cuckoo),
            "NitroCuckoo" => Ok(DsType::NitroCuckoo),
            "FACS" => Ok(DsType::FACS),
            _ => Err(format!("Unrecognized DsType {s}: try HASH, CMS, NitroCMS, SpaceSaving, FDDASH, NitroHash, Cuckoo, NitroCuckoo or FACS"))
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
    #[clap(long, default_value_t = false)]
    pub avoid_mi: bool,
    #[clap(short, long, default_value_t = false)]
    pub verbose : bool,
    #[clap(short, long, default_value_t = false)]
    pub rap : bool,
    #[clap(long, default_value_t = false)]
    pub compare : bool,
    #[clap(long, default_value_t = false)]
    pub compact : bool,
}

#[derive(Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct FlowId {
    srcip : Ipv4Addr,
    dstip : Ipv4Addr,
}

pub fn id_from_line(line: &str) -> Result<FlowId, Box<dyn Error>> {
    let mut parts = line.split_whitespace();
    let srcip: Ipv4Addr = Ipv4Addr::new(
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0)
    );
    let dstip: Ipv4Addr = Ipv4Addr::new(
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0),
        parts.next().unwrap_or_else(|| "0").parse().unwrap_or_else(|_| 0)
    );
    Ok(FlowId { srcip, dstip })
}

// TODO - fix fpdash - currently it is not interesting
fn fpdash_run(_config: Config, _processed: Vec<FlowId>) -> Duration {
//    let num : usize = 2_usize.pow(config.fp_size.into());
//    let counts = DashMap::with_capacity(num);
    let start = Instant::now();
//    for id in &processed {
//        // id.hash(&mut s);
//        //let id : s.finish();
//        let mut s = DefaultHasher::new();
//        id.hash(&mut s);
//        let id = s.finish();
//        if let Some(mut count) = counts.get_mut(&id) {
//            *count+=1;
//        } else {
//            counts.insert(id,1);
//        }
//        if config.time_type == TimeType::RWTIME {
//            counts.get_mut(&id);
//        }
//    }
//    if config.time_type == TimeType::READTIME {
//        start = Instant::now();
//        for id in &processed {
//            let mut s = DefaultHasher::new();
//            id.hash(&mut s);
//            let id = s.finish();
//            counts.get_mut(&id);
//        }
//    }
//    if config.verbose {
//        println!("COUNTS are {:#?}", counts);
//    }
    return start.elapsed();
}

fn hash_run(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts = HashMap::new();
    return generic_time(config, processed, counts);
}

fn hash_accuracy(_config: Config, processed: Vec<FlowId>) -> () {
    let mut baseline = HashMap::new();
    processed.iter().for_each(|id|
        if let Some(count) = baseline.get_mut(id) {
            *count+=1_u32;
        } else {
            baseline.insert(*id,1_u32);
        }
    );
    println!("LENGTH {}", (&processed).len());
    baseline.print_memory_info();
}

fn nitrocms_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, if config.avoid_mi { 1.0 } else { config.sample }, !(config.avoid_mi), ());
    return generic_accuracy(config, processed, counts, true);
}

fn nitrocms_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, if config.avoid_mi { 1.0 } else { config.sample }, !(config.avoid_mi), ());
    return generic_time(config, processed, counts);
}

fn cms_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    //// below is a hack because the corresponding function in the Amadeus CMS implementation is commented out
	//let mut width = f64_to_usize((2.0 / config.error).round());
	//width = max(2, width)
	//	.checked_next_power_of_two()
	//	.expect("Width would be way too large");
	//let k_num = max(
	//	1,
	//	f64_to_usize((1.0/config.confidence).ln().floor()),
	//);
    //println!("Total memory: {}", width * size_of::<u32>() * k_num);
    //// end of hack
    //let counts: CountMinSketch<FlowId,u32> = amadeus_streaming::CountMinSketch::new(config.confidence, config.error, ());
    // The code from Amadeus gave much worse accuracy even though it looks the same, so I am not using the local version
    let counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, 1.0 , !(config.avoid_mi), ());
    generic_accuracy(config, processed, counts, true);
}

fn cms_time(config: Config, processed: Vec<FlowId>) -> Duration {
    //let counts: CountMinSketch<FlowId,u32> = amadeus_streaming::CountMinSketch::new(config.confidence, config.error, ());
    let counts: NitroCMS<FlowId,u32> = NitroCMS::new(config.confidence, config.error, 1.0 , !(config.avoid_mi), ());
    return generic_time(config, processed, counts);
}

fn space_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts: SpaceSaving<FlowId,u32> = SpaceSaving::new(config.error, config.rap);
    return generic_accuracy(config, processed, counts, true);
}

fn space_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts: SpaceSaving<FlowId,u32> = SpaceSaving::new(config.error, config.rap);
    return generic_time(config, processed, counts);
}

fn nitrohash_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts: NitroHash<FlowId,u32> = NitroHash::new(config.sample);
    return generic_accuracy(config, processed, counts, true);
}

fn nitrohash_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts: NitroHash<FlowId,u32> = NitroHash::new(config.sample);
    return generic_time(config, processed, counts);
}

fn cuckoo_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts= CuckooCountingFilter::<DefaultHasher>::with_capacity(processed.len());
    return generic_accuracy(config, processed, counts, true);
}

fn cuckoo_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts= CuckooCountingFilter::<DefaultHasher>::with_capacity(processed.len());
    return generic_time(config, processed, counts);
}

fn nitrocuckoo_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts= if config.compact {
        NitroCuckoo::<DefaultHasher>::with_capacity(processed.len()/((1.0/config.sample).ceil() as usize), config.sample)
    } else {
        NitroCuckoo::<DefaultHasher>::with_capacity(processed.len(), config.sample)
    };
    return generic_accuracy(config, processed, counts, true);
}

fn nitrocuckoo_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts= if config.compact {
        NitroCuckoo::<DefaultHasher>::with_capacity(processed.len()/((1.0/config.sample).ceil() as usize), config.sample)
    } else {
        NitroCuckoo::<DefaultHasher>::with_capacity(processed.len(), config.sample)
    };
    return generic_time(config, processed, counts);
}

fn facs_accuracy(config: Config, processed: Vec<FlowId>) -> () {
    let counts: FACS = FACS::new(config.sample);
    return generic_accuracy(config, processed, counts, true);
}

fn facs_time(config: Config, processed: Vec<FlowId>) -> Duration {
    let counts: FACS = FACS::new(config.sample);
    return generic_time(config, processed, counts);
}

fn generic_accuracy<Q: Sized>(config: Config, processed: Vec<FlowId>, mut counts: Q, memory_info: bool) -> () 
where
Q: ItemIncrement + ItemQuery<Item=u32> + PrintMemoryInfo + std::fmt::Debug, <Q as ItemQuery>::Item: std::fmt::Display, f64: From<<Q as ItemQuery>::Item>
{
    let mut msre_on_arrival = 0.0;
    let mut avgerr_on_arrival = 0.0;
    let mut avgrelerr_on_arrival = 0.0;
    let mut baseline = HashMap::new();
    for id in &processed {
        if let Some(count) = baseline.get_mut(&id) {
            *count+=1;
        } else {
            baseline.insert(id,1);
        }
        counts.item_increment(*id);
        if let Some(count) = baseline.get(&id) {
            if config.verbose {
                println!("{:#?} in Baseline {} in {:?} {}", id, *count, config.ds_type, counts.item_query(*id));
            }
            let item_estimate = f64::from(counts.item_query(*id));
            msre_on_arrival += (item_estimate - f64::from(*count)).powi(2);
            avgerr_on_arrival += abs(item_estimate - f64::from(*count));
            avgrelerr_on_arrival += abs((item_estimate - f64::from(*count))/f64::from(*count));
        }
    }
    println!("LENGTH {}", (&processed).len());
    if memory_info {
        counts.print_memory_info();
    }
    println!("On-Arrival MSRE {}", msre_on_arrival.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap());
    println!("On-Arrival AVGERR {}", avgerr_on_arrival / f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap());
    println!("On-Arrival AVGRELERR {}", avgrelerr_on_arrival / f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap());
    let mut msre_flow = 0.0;
    let mut avgerr_flow = 0.0;
    let mut avgrelerr_flow = 0.0;
    for (id,val) in baseline.iter() {
        let item_estimate = f64::from(counts.item_query(**id));
        msre_flow += (item_estimate - f64::from(*val)).powi(2);
        avgerr_flow += abs(item_estimate - f64::from(*val));
        avgrelerr_flow += abs((item_estimate - f64::from(*val))/f64::from(*val));
               
    }
    println!("Flow MSRE {}", msre_flow.sqrt()/f64::try_from(i32::try_from((baseline).len()).unwrap()).unwrap());
    println!("Flow AVGERR {}", avgerr_flow / f64::try_from(i32::try_from((baseline).len()).unwrap()).unwrap());
    println!("Flow AVGRELERR {}", avgrelerr_flow / f64::try_from(i32::try_from((baseline).len()).unwrap()).unwrap());
    let mut msre_pmw = 0.0;
    let mut avgerr_pmw = 0.0;
    let mut avgrelerr_pmw = 0.0;
    for id in &processed {
        if let Some(count) = baseline.get(&id) {
            let item_real = f64::from(*count);
            let item_estimate = f64::from(counts.item_query(*id));
            msre_pmw += (item_estimate - item_real).powi(2);
            avgerr_pmw += abs(item_estimate - item_real);
            avgrelerr_pmw += abs((item_estimate - item_real)/item_real);
        }
    }  
    println!("PMW MSRE is {}", msre_pmw.sqrt()/f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap());
    println!("PMW AVGERR is {}", avgerr_pmw / f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap()); 
    println!("PMW AVGRELERR is {}", avgrelerr_pmw / f64::try_from(i32::try_from((&processed).len()).unwrap()).unwrap()); 
}

fn generic_time<Q: Sized>(config: Config, processed: Vec<FlowId>, mut counts: Q) -> Duration
where
Q: ItemIncrement + ItemQuery + PrintMemoryInfo + std::fmt::Debug,
{
    println!("LENGTH {}", (&processed).len());
    let mut start = Instant::now();
    for id in &processed {
        counts.item_increment(*id);
        if config.time_type == TimeType::RWTIME {
            counts.item_query(*id);
        }
    }
    if config.time_type == TimeType::READTIME {
        start = Instant::now();
        //for id in &processed {
        processed.iter().for_each(|id|
            {counts.item_query(*id);});
        //}
    }
    if config.verbose {
        println!("COUNTS are {:#?}", counts);
    }
    return start.elapsed();
}

fn preprocess_contents(contents: String) -> Vec<FlowId> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if let Ok(id) = id_from_line(line) {
            result.push(id);
        }
    }
    result
}

/// Perform measurements according to the specified parameters.
/// Most importanly, timing measurements OR accuracy comparisson and memory usage
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //println!("{:#?}({}) {:#?} for FILE: {}", config.ds_type, config.rap, config.time_type, config.file_path);
    println!("TRACE {}", config.file_path);
    if config.compare {
        println!("TEST COMPARE")
    } else {
        println!("TEST {:#?}", config.time_type);
    }
    if config.rap {
        println!("DSTYPE {:#?}-RAP", config.ds_type);
    } else if config.compact {
        println!("DSTYPE {:#?}-SMALL", config.ds_type);
    } else if config.avoid_mi {
        println!("DSTYPE CMS-NOMI");
    } else {
        println!("DSTYPE {:#?}", config.ds_type);
    }
    let contents = fs::read_to_string(config.file_path.clone())?;
    if config.verbose {
        println!("PREPROCESSING DONE");
    }
    let processed = preprocess_contents(contents);
    #[cfg(feature = "stats")]
    let mem_allocated:usize;
    #[cfg(feature = "stats")]
    let total_allocated:usize;
    #[cfg(feature = "stats")]
    let max_allocated:usize;
    #[cfg(feature = "stats")]
    {
        mem_allocated = ALLOCATOR.allocated();
        println!("{}", mem_allocated);
        total_allocated = ALLOCATOR.total_allocated();
        println!("{}", total_allocated);
        max_allocated = ALLOCATOR.max_allocated();
        println!("{}", max_allocated);
    }
    if config.compare {
        match config.ds_type {
            DsType::HASH => hash_accuracy(config, processed),
            DsType::CMS => if config.avoid_mi {
                                nitrocms_accuracy(config, processed)
                            } else {
                                cms_accuracy(config, processed)
                            },
            DsType::NitroCMS => nitrocms_accuracy(config, processed),
            DsType::FPDASH => (),
            DsType::SpaceSaving => space_accuracy(config, processed),
            DsType::NitroHash => nitrohash_accuracy(config, processed),
            DsType::Cuckoo => cuckoo_accuracy(config, processed),
            DsType::NitroCuckoo => nitrocuckoo_accuracy(config, processed),
            DsType::FACS => facs_accuracy(config, processed),
            //_ => (),
        };
    } else {
        let elapsed_time  = match config.ds_type {
            DsType::HASH => hash_run(config, processed),
            DsType::CMS => cms_time(config, processed),
            DsType::NitroCMS => nitrocms_time(config, processed),
            DsType::FPDASH => fpdash_run(config, processed),
            DsType::SpaceSaving => space_time(config, processed),
            DsType::NitroHash => nitrohash_time(config, processed),
            DsType::Cuckoo => cuckoo_time(config, processed),
            DsType::NitroCuckoo => nitrocuckoo_time(config, processed),
            DsType::FACS => facs_time(config, processed),
            //_ => (),
        };
        println!("TIMEms = {}", elapsed_time.as_micros());
    }
    #[cfg(feature = "stats")]
    {
        println!("{}", ALLOCATOR.allocated());
        println!("{}", ALLOCATOR.total_allocated());
        println!("{}", ALLOCATOR.max_allocated());
        println!("{}", ALLOCATOR.max_allocated() - mem_allocated);
    }
    println!("END");
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