pub trait IsFn {
    fn is_fn (&self, fn_sig: &str) -> bool;
}

impl IsFn for String {
    fn is_fn(&self, fn_sig: &str) -> bool {
        if self.to_lowercase().contains(&fn_sig.to_lowercase()) {
            println!("Found {}", fn_sig);
            return true;
        } else {
            println!("Not found {}", fn_sig);
            return false;
        }
    }
}