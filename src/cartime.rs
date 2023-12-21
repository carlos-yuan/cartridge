use chrono::Local;

const TIME_FORMAT: &str = "%Y%m%d%H%M%S%.3f%z";

pub fn now()->u64 {
    let mut t=Local::now().format(TIME_FORMAT).to_string();
    t.remove(14);
    t.remove(17);
    t.remove(19);
    t.remove(19);
    t.parse().unwrap()
}