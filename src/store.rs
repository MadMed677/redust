use std::collections::HashMap;

use crate::subscription::{SubscriptionToken, UnsubscribeError};
use crate::{Reducer, Subscription};

pub struct Store<State, Action> {
    reducer: Reducer<State, Action>,
    state: State,
    subscriptions: HashMap<SubscriptionToken, Subscription<State>>,
    subscriptions_index: SubscriptionToken,
}

impl<State, Action> Store<State, Action> {
    /// Creates a new store
    pub fn new(reducer: Reducer<State, Action>, initial_state: State) -> Self {
        Self {
            reducer,
            state: initial_state,
            subscriptions: HashMap::new(),
            subscriptions_index: 0,
        }
    }

    /// Returns the current state tree of your application.
    /// It is equal to the last value returned by the store's reducer.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Dispatches an action. This is the only way to trigger a state change
    ///
    /// ## Example (simple action type)
    /// ```rust
    /// use redust::Store;
    ///
    /// type MyStore = u8;
    ///
    /// #[derive(Debug)]
    /// enum MyAction {
    ///     Increment,
    /// };
    ///
    /// fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
    ///     match action {
    ///         MyAction::Increment => state + 1,
    ///     }
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    ///
    /// store.dispatch(MyAction::Increment);
    ///
    /// assert_eq!(*store.state(), 1);
    /// ```
    ///
    /// ## Example (complex action type)
    /// Also you may add additional parameters into the action and handle them
    /// in the `reducer` function
    ///
    /// ```rust
    /// use redust::Store;
    ///
    /// type MyStore = u8;
    ///
    /// #[derive(Debug)]
    /// enum MyAction {
    ///     // Increment on a specific value
    ///     IncrementBy(u8),
    /// };
    ///
    /// fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
    ///     match action {
    ///         MyAction::IncrementBy(value) => state + value,
    ///     }
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    ///
    /// store.dispatch(MyAction::IncrementBy(10));
    ///
    /// assert_eq!(*store.state(), 10);
    /// ```
    pub fn dispatch(&mut self, action: Action) -> &mut Store<State, Action> {
        self.state = (&self.reducer)(self.state(), &action);

        self.subscriptions.iter().for_each(|(_, subsciber)| {
            subsciber(&self.state);
        });

        self
    }

    /// Subscribes a callback to any change of the state.
    ///
    /// Subscriptions will be called, whenever an action is dispatched.
    ///
    /// ## Example
    /// ```rust
    /// use redust::Store;
    ///
    /// type MyStore = u8;
    ///
    /// #[derive(Debug)]
    /// enum MyAction {};
    ///
    /// fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
    ///     1
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    ///
    /// store.subscribe(|state| {
    ///     // Do any code here. This place will be called each time
    ///     //  when anything will change in the hole storage
    ///     assert_eq!(*state, 1);
    /// });
    /// ```
    pub fn subscribe(&mut self, func: Subscription<State>) -> SubscriptionToken {
        let subscription_token = self.subscriptions_index;
        self.subscriptions.insert(subscription_token, func);

        // Increment subscriptions token
        self.subscriptions_index += 1;

        subscription_token
    }

    /// Unsubscribes a callback by `SubscriptionToken` which will returned from the `subscribe` method
    ///
    /// ## Example
    /// ```rust
    /// use redust::Store;
    ///
    /// type MyStore = u8;
    ///
    /// #[derive(Debug)]
    /// enum MyAction {};
    ///
    /// fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
    ///     1
    /// }
    ///
    /// let mut store = Store::new(reducer, 0);
    ///
    /// let subscription_token = store.subscribe(|state| {
    ///     assert_eq!(*state, 1);
    /// });
    ///
    /// store.unsubscribe(subscription_token);
    /// ```
    pub fn unsubscribe(
        &mut self,
        subscription_token: SubscriptionToken,
    ) -> Result<(), UnsubscribeError> {
        // Nothing in the subscription
        if let None = self.subscriptions.remove(&subscription_token) {
            return Err(UnsubscribeError::WrongToken(subscription_token));
        }

        Ok(())
    }
}
