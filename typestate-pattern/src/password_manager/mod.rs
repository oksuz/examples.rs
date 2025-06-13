use std::collections::HashMap;

// for locked state
pub struct Locked;
//for unlucked state
pub struct Unlocked;

pub struct PasswordManager<State = Locked> {
    passwords: HashMap<String, String>,
    master_password: String,
    state: std::marker::PhantomData<State> // zero sized type
}

// locked password manager impl
impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager { 
          passwords: self.passwords, 
          master_password: self.master_password, 
          state: std::marker::PhantomData::<Unlocked> 
        }
    }
}

// locked password manager impl
impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager { 
          passwords: self.passwords, 
          master_password: self.master_password, 
          state: std::marker::PhantomData::<Locked> 
        }
    }

    pub fn add_new(&mut self, uname: String, password: String) {
      self.passwords.insert(uname, password);
    }

    pub fn list(&self) -> &HashMap<String, String> {
      &self.passwords
    }
}

// common password manager impl
impl<State> PasswordManager<State> {
  pub fn version(&self) -> usize {
    1
  }
}

// default imp
impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager { passwords: Default::default(), master_password: "123456".to_string(), state: Default::default() }
    }
}