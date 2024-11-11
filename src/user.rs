
 pub(crate) struct User {
    pub(crate) active: bool,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) sign_in_count: u64,
}

 impl User {
     
    pub fn show_name(self) {
         println!("Show name: {}", self.username)
     }
 }