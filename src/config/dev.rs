pub mod config {
    pub const MODE: &str = "DEVELOPMENT MODE";
    pub const DATABASE_URL: &str = "postgresql://superuser:superpassword@localhost:5432/user"; // your dev enviroment database url
    pub const PORT: u16 = 8888; // your dev enviroment port
}