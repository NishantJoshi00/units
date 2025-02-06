pub trait Users {
    fn get_user(&self, username: &str, password: &str) -> anyhow::Result<Option<User>>;
    fn validate_user_id(&self, user_id: &str) -> anyhow::Result<bool>;
    fn create_user(&self, user: User, password: &str) -> anyhow::Result<bool>;
}

impl Users for () {
    fn get_user(&self, username: &str, _password: &str) -> anyhow::Result<Option<User>> {
        Ok(Some(User {
            user_id: "root".to_string(),
            username: username.to_string(),
        }))
    }

    fn validate_user_id(&self, _user_id: &str) -> anyhow::Result<bool> {
        Ok(true)
    }

    fn create_user(&self, _user: User, _password: &str) -> anyhow::Result<bool> {
        Ok(true)
    }
}

pub struct User { user_id: String, username: String, }
