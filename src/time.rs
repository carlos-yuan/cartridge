use chrono::{Local, Datelike, NaiveDateTime, DateTime, TimeZone};

pub const   SECOND   :u64 = 1000;
pub const   SECOND30   :u64 = 30*SECOND;
pub const 	MINUTE   :u64 = 60 * SECOND;
pub const 	MINUTE3  :u64 = 3 * MINUTE;
pub const 	MINUTE5  :u64 = 5 * MINUTE;
pub const 	MINUTE10 :u64 = 10 * MINUTE;
pub const 	MINUTE15 :u64 = 15 * MINUTE;
pub const 	MINUTE20 :u64 = 20 * MINUTE;
pub const 	MINUTE30 :u64 = 30 * MINUTE;
pub const 	HOUR     :u64 = 60 * MINUTE;
pub const 	DAY      :u64 = 24 * HOUR;
pub const 	WEEK     :u64 = 7 * DAY;
pub const 	MONTH    :u64 = 30 * DAY;

pub fn millis()->u64{
    Local::now().timestamp_millis() as u64
}

pub fn nanos()->u64{
    Local::now().timestamp_nanos_opt().unwrap() as u64
}

pub fn to_yyyyMM(time:u64)->u32{
    let secs=time/1000;
    let nsecs=(time-secs)*1000_000;
    let dt=NaiveDateTime::from_timestamp_opt((secs+8*3600) as i64,nsecs as u32).unwrap();
    println!("{}",dt.timestamp());
    (dt.year()*100) as u32+dt.month()
}

pub fn to_yyyyMMdd(time:u64)->u32{
    let secs=time/1000;
    let nsecs=(time-secs)*1000_000;
    let dt=NaiveDateTime::from_timestamp_opt((secs+8*3600) as i64,nsecs as u32).unwrap();
    (dt.year()*10000) as u32+dt.month()*100+dt.day()
    
}

pub fn now_yyyyMMdd()->u32{
    let dt=Local::now();
    (dt.year()*10000) as u32+dt.month()*100+dt.day()
}

pub fn now_yyyyMM()->u32{
    let dt=Local::now();
    (dt.year()*100) as u32+dt.month()
}

pub fn now_fmt(fmt:&str)->String{
    Local::now().format(fmt).to_string()
}

pub fn to_fmt(time:u64,fmt:&str)->String{
    let dt=NaiveDateTime::from_timestamp_opt(time as i64,0).unwrap();
    dt.format(fmt).to_string()
}

pub fn parse(time:String,fmt:String)->u64{
    let mut time=time.clone();
    let mut fmt=fmt.clone();
    if !fmt.contains("%H"){
        fmt=format!("{} {}",fmt,"%H");
        time=format!("{} {}",time,"0");
    }
    if !fmt.contains("%M"){
        fmt=format!("{} {}",fmt,"%M");
        time=format!("{} {}",time,"0");
    }
    if !fmt.contains("%z"){
        fmt=format!("{} {}",fmt,"%z");
        time=format!("{} {}",time,Local::now().format("%z"));
    }
    println!("{}:{}",fmt,time);
    let dt=DateTime::parse_from_str(time.as_str(), fmt.as_str());
    (dt.unwrap().timestamp_millis() as u64)//-8*3600_000 
}

