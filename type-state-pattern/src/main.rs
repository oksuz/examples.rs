use password_manager::PasswordManager;

pub mod password_manager;

fn main() {
    let mgr = PasswordManager::new();
    let mgr = mgr.unlock("123456".to_string());
    let mgr = mgr.lock();
    // mgr.lock() // compile time error - cannot lock unlocked
    
}
