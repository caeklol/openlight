use crate::provider::Provider;


pub struct View {
    provider: Provider
}

impl View {
    pub fn start(self) {
        
    }

    pub fn init(provider: Provider) -> Self {
        return Self {
            provider
        };
    }
}
