pub struct User {
  username: String,
  password: String,
}

impl User {
  pub fn new(username: &str, password: &str) -> User {
    User {
      username: username.to_string(),
      password: password.to_string()
    }
  }

  pub fn matches(&self, username: &str, password: &str) -> bool {
    username == self.username && password == self.password
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn user_matches() {
    let user = User::new("admin", "123456");
    let result = user.matches("admin", "123456");
    assert_eq!(result, true);
    let result = user.matches("admin", "654321");
    assert_eq!(result, false);
  }
}
