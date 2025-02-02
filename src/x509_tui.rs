use crate::x509::X509;
use ratatui::widgets::ListState;

pub struct X509TUIList {
    pub items: Vec<X509>,
    pub state: ListState,
}

impl X509TUIList {
    pub fn new(x509s: Vec<X509>) -> Self {
        X509TUIList {
            items: x509s,
            state: ListState::default(),
        }
    }
}
