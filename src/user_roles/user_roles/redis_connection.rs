
/// Module for getting and inserting roles in the redis cache.
pub mod roles {

    use redis::{self, Commands, RedisResult};

    /// Gets a role based on a ```jwt_token```
    ///
    /// # Arguments
    ///
    ///* `jwt_token` - A String struct that contains the ```jwt_token```
    ///
    /// # Returns
    /// The role that's tied to the ```jwt_token```
    pub fn get_role(jwt_token: String) -> redis::RedisResult<String> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut con = client.get_connection()?;
        let role = con.get(jwt_token);
        return role
    }

    /// Inserts a role that's indexed by a ```jwt_token```
    ///
    /// # Arguments
    ///
    /// * `jwt_token` - A String struct that contains the ```jwt_token```
    /// * `role` - A String struct that contains the role of the user
    pub fn insert_role(jwt_token: String, role: &String) -> RedisResult<String> {
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut con = client.get_connection()?;
        let _ = con.set(&jwt_token, role)?;
        let role = con.get(jwt_token);
        return role
    }

}
