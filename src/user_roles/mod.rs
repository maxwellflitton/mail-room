
/// Defines the roles of a user and manages the storing and lookup of these roles.
pub mod user_roles {

    mod redis_connection;
    use serde_json::json;

    /// Data that describes the result of reading or writing a role with a `jwt_token`.
    pub struct UserRole {
        /// role of the user
        pub role: String,
        /// if the read or write was successful or not
        pub success: bool,
        /// error message if there's a failure in the reading or writing to redis
        pub message: String
    }

    impl UserRole {

        /// Creates a new user role and inserts it into redis.
        ///
        ///  # Arguments
        ///
        /// * `jwt_token` - user jwt token that is bound to a role
        /// * `role` - role that the user is assigned
        ///
        /// # Returns
        /// The status of the write process to redis
        pub fn new(jwt_token: String, role: String) -> UserRole {

            let result: UserRole = match redis_connection::roles::insert_role(jwt_token, &role) {
                Ok(result) => UserRole {role: result, success: true, message: String::from("no problems")},
                Err(e) => UserRole {role: String::from("None"), success: false, message: String::from(e.category())}
            };
            return result
        }

        /// Gets a role from an existing `jwt_token` stored in redis.
        ///
        /// # Arguments
        /// * `jwt_token` - an existing user jwt token that is bound to a role
        ///
        /// # returns
        /// The status of the read process to redis
        pub fn get(jwt_token: String) -> UserRole {

            let result: UserRole = match redis_connection::roles::get_role(jwt_token) {
                Ok(result) => UserRole {role: result, success: true, message: String::from("no problems")},
                Err(e) => UserRole {role: String::from("None"), success: false, message: String::from(e.category())}
            };
            return result
        }

        /// Packs the data of the UserRole struct into json, and then a String for response.
        ///
        /// # Arguments
        /// None
        ///
        /// # returns
        /// packed UserRole data for hyper body in a hyper response
        pub fn pack(self) -> String {

            let json_buffer = json!({
                "role": self.role,
                "success": self.success,
                "message": self.message
            });

            return serde_json::to_string(&json_buffer).unwrap()
        }
    }
}
