use clap::Parser;
use tcp::tcp_sniffer::tcp_sniffer;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
struct Arguments {
    to: String,
    from: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    // tcp_sniffer(&args.from, &args.to).await?;
    tcp_sniffer(&args.from, &args.to)?;
    Ok(())
}
