pub mod config {
    pub const MODE: &str = "PRODUCTION MODE";
    pub const DATABASE_URL: &str = "postgresql://superuser:superpassword@localhost:5432/user"; // your prod enviroment database url
    pub const PORT: u16 = 8080; // your prod enviroment port
}