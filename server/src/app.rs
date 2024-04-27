use tokio::signal;

pub struct App {
    date: u64,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            date: 0,
        })
    }


    pub async fn run(&mut self) -> anyhow::Result<()> {
        self.starting().await?;
        return tokio::select! {
            result = self.running() => result,
            result = signal::ctrl_c() => {
                println!("shutting down server");
                result.map_err(|e| anyhow::anyhow!("error: {:?}", e))
            },
        };
    }

    async fn starting(&mut self) -> anyhow::Result<()> {
        println!("starting server");
        Ok(())
    }

    async fn running(&mut self) -> anyhow::Result<()> {
        let duration = std::time::Duration::from_millis(50); // 20 Hz
        let mut interval = tokio::time::interval(duration);

        loop {
            interval.tick().await;
            self.date += 1;
            self.tick().await?;
        }
    }

    pub async fn tick(&mut self) -> anyhow::Result<()> {
        println!("tick {}", self.date);
        Ok(())
    }
}
