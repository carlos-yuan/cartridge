use rand::Rng;
use uuid::Uuid;

pub fn number(prefix:&str,length:u8)->u64{
    if length>18{
        panic!("max more than lenghn 18");
    }
    let mut min="1".to_string();
    let mut max="9".to_string();
    if prefix!=""{
        min=prefix.to_string();
        max=prefix.to_string();
    }
    for _ in 0..length-1 {
        min+=&"0".to_string();
        max+=&"9".to_string();
    }
    let min:u64=min.parse().unwrap();
    let max:u64=max.parse().unwrap();
    rand::thread_rng().gen_range(min,max)
}

pub fn number_str(prefix:&str,length:usize)->String{
    let mut min="1".to_string();
    let mut max="9".to_string();
    for _ in 0..18 {
        min+=&"0".to_string();
        max+=&"9".to_string();
    }
    let min:u64=min.parse().unwrap();
    let max:u64=max.parse().unwrap();
    let mut res=prefix.to_string();
    while res.len()<length{
        let num=rand::thread_rng().gen_range(min,max);
        res+=&num.to_string();
    }
    if res.len()>length{
        res=(&res[0..length]).into();
    }
    res
}

pub fn str(length:usize)->String{
    let mut res="".to_string();
    while res.len()<length {
        res+=&Uuid::new_v4().to_string().replace("-", "");
    }
    if res.len()>length{
        res=(&res[0..length]).into();
    }
    res
}