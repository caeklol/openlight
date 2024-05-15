use openlight::provider::Provider;
use openlight::view::View;

pub fn main() {
    let provider = Provider::init();
    let view = View::init(provider);
    view.start();
}
