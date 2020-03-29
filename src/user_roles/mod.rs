

pub mod user_roles {

    mod redis_connection;
    use std::any::type_name;

    pub struct UserRole {
        pub role: String,
        pub success: bool,
        pub message: String
    }

    impl UserRole {

        pub fn new(jwt_token: String, role: String) -> UserRole {

            let result: UserRole = match redis_connection::roles::insert_role(jwt_token, &role) {
                Ok(result) => UserRole {role: result, success: true, message: String::from("no problems")},
                Err(e) => UserRole {role: String::from("None"), success: false, message: String::from(e.category())}
            };
            return result
        }

        pub fn get(jwt_token: String) -> UserRole {

            let result: UserRole = match redis_connection::roles::get_role(jwt_token) {
                Ok(result) => UserRole {role: result, success: true, message: String::from("no problems")},
                Err(e) => UserRole {role: String::from("None"), success: false, message: String::from(e.category())}
            };
            return result
        }

    }

}
