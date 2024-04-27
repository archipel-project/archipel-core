mod server_shell;
mod logical_server;
use server_shell::ServerShell;


fn main() -> anyhow::Result<()> {
    ServerShell::init()?.run()
}
