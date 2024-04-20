use validator::Validate;

#[derive(Debug, Validate)]
pub struct Config {
    /// The address to bind the server to.
    /// TODO: add a validator for the address
    #[validate(length(min = 1, message = "The bind address must not be empty"))]
    pub(crate) bind: String,

    /// Message Of The Day, shown to the client when they connect.
    pub(crate) motd: String,

    /// The maximum number of players that are displayed in the server list.
    #[validate(range(
        min = 1,
        message = "The maximum number of players must be greater than 0"
    ))]
    pub(crate) show_max_players: u32,

    /// If true, the server will allow only online-mode clients to connect.
    pub(crate) online_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:25565".into(),
            motd: "<orange>An Archipel Core server".into(),
            online_mode: false,
            show_max_players: 500,
        }
    }
}
