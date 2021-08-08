use std::collections::HashMap;
use std::fmt::Debug;

use crate::subscription::{SubscriptionToken, UnsubscribeError};
use crate::{Reducer, Subscription};

pub struct Store<State, Action> {
    reducer: Reducer<State, Action>,
    state: State,
    subscriptions: HashMap<SubscriptionToken, Subscription<State>>,
    subscriptions_index: SubscriptionToken,
}

impl<State, Action> Store<State, Action>
where
    Action: Debug,
    State: Debug,
{
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
    // pub fn subscribe<'a>(&'a mut self, func: Subscription<State>) -> impl FnMut() -> () + 'a {
    pub fn subscribe(&mut self, func: Subscription<State>) -> SubscriptionToken {
        let subscription_token = self.subscriptions_index;
        self.subscriptions.insert(subscription_token, func);

        // Increment subscriptions token
        self.subscriptions_index += 1;

        subscription_token
    }

    /// Unsubscribes a callback by `SubscriptionToken` which will returned
    ///  from the `subscribe` method
    /// # Example
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

#[cfg(test)]
mod store {
    mod dispatch {
        use crate::store::Store;

        #[test]
        fn dispatch_should_cause_updates_in_the_store() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                Increment,
                Decrement,
            }
            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                match action {
                    MyAction::Increment => state + 1,
                    MyAction::Decrement => state - 1,
                }
            }

            let mut store = Store::new(reducer, 0);

            // Should increase the value to 1
            store.dispatch(MyAction::Increment);
            assert_eq!(store.state, 1);

            // Should increase the value to 2
            store.dispatch(MyAction::Increment);
            assert_eq!(store.state, 2);

            // Should decrease the value to 1
            store.dispatch(MyAction::Decrement);
            assert_eq!(store.state, 1);
        }

        #[test]
        fn should_increment_or_decrement_the_reducer_data_by_specific_value() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                // Increment on specific value
                IncrementBy(u8),

                // Decrement on specific value
                DecrementBy(u8),
            }

            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                match action {
                    MyAction::IncrementBy(value) => state + value,
                    MyAction::DecrementBy(value) => state - value,
                }
            }

            let mut store = Store::new(reducer, 0);

            // Should increate the value to 5
            store.dispatch(MyAction::IncrementBy(5));
            assert_eq!(store.state, 5);

            // Should decrement the value to 3
            store.dispatch(MyAction::DecrementBy(2));
            assert_eq!(store.state, 3);

            // Should decrement the value to 4
            store.dispatch(MyAction::IncrementBy(1));
            assert_eq!(store.state, 4);
        }

        #[test]
        fn dispatch_should_called_multiple_times_when_chain() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                Increment,
                Decrement,
            }
            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                match action {
                    MyAction::Increment => state + 1,
                    MyAction::Decrement => state - 1,
                }
            }

            let mut store = Store::new(reducer, 0);

            // Should increase the value to 3
            store
                .dispatch(MyAction::Increment)
                .dispatch(MyAction::Increment)
                .dispatch(MyAction::Increment);
            assert_eq!(store.state, 3);
        }
    }

    mod subscription {
        use crate::store::{Store, UnsubscribeError};

        #[test]
        fn should_call_one_subscription_when_dispatch_called() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                Increment,
            }
            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                match action {
                    MyAction::Increment => state + 1,
                }
            }

            let mut store = Store::new(reducer, 0);
            let _ = store.subscribe(|state| {
                assert_eq!(*state, 1);
            });

            store.dispatch(MyAction::Increment);
        }

        #[test]
        fn should_not_call_subscriber_when_we_unsubscribed() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                Increment,
            }
            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                1
            }

            let mut store = Store::new(reducer, 0);
            let subscription_token = store.subscribe(|state| {
                assert_eq!(*state, 1);
            });

            store.dispatch(MyAction::Increment);

            store.unsubscribe(subscription_token);

            store.dispatch(MyAction::Increment);
        }

        #[test]
        fn should_throw_an_error_when_unsubscribe_will_call_with_wrong_token() {
            type MyStore = u8;

            #[derive(Debug)]
            enum MyAction {
                Increment,
            }
            fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
                1
            }

            let mut store = Store::new(reducer, 0);
            store.subscribe(|state| {
                assert_eq!(*state, 1);
            });

            store.dispatch(MyAction::Increment);

            let wrong_token = 99;
            let result = store.unsubscribe(wrong_token);

            if let Err(token) = result {
                assert_eq!(token, UnsubscribeError::WrongToken(wrong_token));
            } else {
                panic!("Have to be true");
            }
        }
    }
}
