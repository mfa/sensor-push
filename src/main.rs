mod gpu_sensor;
mod push;
mod data;

use clap::Parser;

use gpu_sensor::sensor_values;
use push::send;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sensortype
    #[arg(short, long, default_value = "nvidia_gpu")]
    stype: String, // FIXME: use enum later

    /// identifier
    #[arg(short, long)]
    id: String,

    /// url to push to
    #[arg(short, long)]
    url: String,
}

fn main() {
    let args = Args::parse();

    if args.stype == "nvidia_gpu" {
        let dataset = sensor_values().expect("no GPU found");
        send(dataset, args.url, &args.id);
    }
}
