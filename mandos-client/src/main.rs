mod mandos_client;
mod networking;

use mandos_client::MandosClient;

fn main() {
    env_logger::init();

    let app = MandosClient::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
