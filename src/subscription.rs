pub type Subscription<State> = fn(&State);

pub type SubscriptionToken = u8;

#[derive(Debug, PartialEq)]
pub enum UnsubscribeError {
    WrongToken(SubscriptionToken),
}

impl std::error::Error for UnsubscribeError {}
impl std::fmt::Display for UnsubscribeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnsubscribeError::WrongToken(token) => {
                write!(f, "Cannot find the subscription by token: {}", token)
            }
        }
    }
}
