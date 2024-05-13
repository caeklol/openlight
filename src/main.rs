use openlight::provider::Provider;
use openlight::view::View;

pub fn main() {
    let provider = Provider::init();
    println!("{:?}", provider.find("Disc"));
}
