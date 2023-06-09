// https://github.com/letsgetrusty/generics_and_zero_sized_types/blob/master/src/main.rs

use std::collections::HashMap;
use std::marker::PhantomData;

struct Locked;
struct Unlocked;

struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass: master_pass,
            passwords: self.passwords,
            state: PhantomData,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

impl<State> PasswordManager<State> {
    #[allow(dead_code)]
    pub fn encryption(&self) -> String {
        todo!()
    }

    #[allow(dead_code)]
    pub fn version(&self) -> String {
        todo!()
    }
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass: master_pass,
            passwords: Default::default(),
            state: PhantomData,
        }
    }
}

fn main() {
    let pm = PasswordManager::new("s3kr3t".to_owned());
    let mut pm = pm.unlock("s3kr3t".to_owned());
    pm.add_password("john".to_string(), "pass123".to_string());
    pm.add_password("marie".to_string(), "M@r13!".to_string());
    for (key, value) in pm.list_passwords() {
        println!("{} / {}", key, value);
    }
    
    pm.lock();
}
