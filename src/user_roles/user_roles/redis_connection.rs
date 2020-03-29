

pub mod roles {

    use redis::{self, Commands, RedisResult};

    pub fn get_role(jwt_token: String) -> redis::RedisResult<String> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut con = client.get_connection()?;
        let role = con.get(jwt_token);
        return role
    }

    pub fn insert_role(jwt_token: String, role: &String) -> RedisResult<String> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut con = client.get_connection()?;
        let _ = con.set(&jwt_token, role)?;
        let role = con.get(jwt_token);
        return role
    }

}
