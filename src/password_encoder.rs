use md5;

pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: String) -> String {
        let digest = md5::compute(raw_password);
        format!("{:x}", digest)
    }
}
