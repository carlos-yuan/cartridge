use redis_cluster_async::redis::{ToRedisArgs, FromRedisValue, RedisError, self};
use serde::de::Visitor;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::fmt;
use redis_cluster_async::{Client,redis::Script, redis::cmd};

pub const   SECOND   :usize = 1;
pub const 	MINUTE   :usize = 60 * SECOND;
pub const 	MINUTE3  :usize = 3 * MINUTE;
pub const 	MINUTE5  :usize = 5 * MINUTE;
pub const 	MINUTE10 :usize = 10 * MINUTE;
pub const 	MINUTE15 :usize = 15 * MINUTE;
pub const 	MINUTE20 :usize = 20 * MINUTE;
pub const 	MINUTE30 :usize = 30 * MINUTE;
pub const 	HOUR     :usize = 60 * MINUTE;
pub const 	DAY      :usize = 24 * HOUR;
pub const 	WEEK     :usize = 7 * DAY;
pub const 	MONTH    :usize = 30 * DAY;

pub struct RedisConnection{
    client:Client
}

impl RedisConnection {
    pub fn new(nodes: Vec<String>) -> Self {
        let client = Client::open(nodes).unwrap();
        RedisConnection { client }
    }

    pub async fn set<K: ToRedisArgs, V: ToRedisArgs>(&self,key: K, value: V, seconds: usize) -> Result<bool,RedisUtilError> {
        let ref mut con=self.client.clone().get_connection().await?;
        let res:String= cmd("SETEX").arg(key).arg(seconds).arg(value).query_async(con).await?;
        if res=="OK"{
            Ok(true)
        }else{
            Err(res.into())
        }
    }

    pub async fn get<K: ToRedisArgs, V: FromRedisValue>(&self,key: K) -> Result<V,RedisUtilError> {
        let ref mut con=self.client.clone().get_connection().await?;
        let res:V= cmd("GET").arg(key).query_async(con).await?;
        Ok(res)
    }

    pub async fn ttl<K: ToRedisArgs, V: FromRedisValue>(&self,key: K) -> Result<V,RedisUtilError> {
        let ref mut con=self.client.clone().get_connection().await?;
        let res:V= cmd("TTL").arg(key).query_async(con).await?;
        Ok(res)
    }

    pub async fn del_ref(&self,key:&str) -> Result<u32,RedisUtilError> {
        let script = Script::new(
            r"
            local setKey=redis.call('smembers',KEYS[1]) 
			for key,value in ipairs(setKey) do redis.call('del',value)  end return 1
            ",
        );
        let ref mut con=self.client.clone().get_connection().await?;
        let val:u32=script.key(key).invoke_async(con).await?;
        Ok(val)
    }

    pub async fn set_ref(&self,key:&str) -> Result<u32,RedisUtilError> {
        let script = Script::new(
            r"
            if (redis.call('setex',KEYS[1],ARGV[1],ARGV[2]) == 'OK') then 
				redis.call('sadd',KEYS[2],KEYS[1]) 
				return 1 
			else 
				return 0 
			end
            ",
        );
        let ref mut con=self.client.clone().get_connection().await?;
        let val:u32=script.key(key).invoke_async(con).await?;
        if val==0{
            Err(RedisUtilError::UtilError(format!("return {}",val)))
        }else{
            Ok(val)
        }
    }

    pub async fn limit(&self,key:&str,time:u32,count:u32) -> Result<bool,RedisUtilError> {
        let script = Script::new(
            r"local times = redis.call('incr',KEYS[1])
			 if times == 1 then
			   redis.call('expire',KEYS[1], ARGV[1])
			 end
			  if times > tonumber(ARGV[2]) then
			    return 0
			  end
			 return 1",
        );
        let ref mut con=self.client.clone().get_connection().await?;
        let val:u32=script.key(key).arg(time).arg(count).invoke_async(con).await?;
        if val==1{
            Ok(true)
        }else{
            Ok(false)
        }
    }

    pub async fn lock(&self,key:&str,out:usize) -> Result<bool,RedisUtilError> {
        let script = Script::new(
            r"if (redis.call('setnx',KEYS[1],ARGV[1]) == 1) then
			   redis.call('expire',KEYS[1],ARGV[2]) 
			   return 1 
			end 
			if (redis.call('ttl',KEYS[1]) == -1) then 
			   if(redis.call('setex',KEYS[1],ARGV[1],ARGV[2])>0) then 
			       return 1 
			   end 
			end 
			return 0",
        );
        let ref mut con=self.client.clone().get_connection().await?;
        let val:u8=script.key(key).arg(out).invoke_async(con).await?;
        if val==1{
            Ok(true)
        }else{
            Ok(false)
        }
    }
}

#[derive(Debug)]
pub enum RedisUtilError {
  RedisError(RedisError),
  RedisNil,
  UtilError(String),
}

impl std::error::Error for RedisUtilError {}

impl fmt::Display for RedisUtilError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        RedisUtilError::RedisError(e)=> e.fmt(f),
        RedisUtilError::RedisNil=> write!(f, "return nil"),
        RedisUtilError::UtilError(e) => write!(f, "Util err {}",e),
    }
  }
}

impl From<RedisError> for RedisUtilError {
    fn from(err: RedisError) -> RedisUtilError {
        if err.to_string().contains("response was nil"){
            RedisUtilError::RedisNil
        }else{
            RedisUtilError::RedisError(err)
        }
    }
}

impl From<String> for RedisUtilError {
    fn from(err: String) -> RedisUtilError {
        RedisUtilError::UtilError(err)
    }
}

impl Serialize for RedisUtilError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct ErrorVisitor;

impl<'de> Visitor<'de> for ErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: std::error::Error,
    {
        Ok(v.to_string())
    }
}

impl<'de> Deserialize<'de> for RedisUtilError {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let r = deserializer.deserialize_string(ErrorVisitor)?;
        return Ok(RedisUtilError::from(r));
    }
}
