use clap::Parser;
use stream_url::api::{
    bilibili::{self, model::QNData},
    GetUrl,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    roomd_id: i32,
    #[clap(short, long, arg_enum, default_value_t=QNData::QSource)]
    qn_data: QNData,
    #[clap(short, long)]
    all: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let room = bilibili::req::StreamRoom::new(args.roomd_id, args.qn_data);
    let req = room.get_stream_url();
    match req {
        Ok(url) => {
            if args.all {
                for i in url {
                    println!("{}", i);
                }
            } else {
                println!("{}", url.first().expect(""));
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
