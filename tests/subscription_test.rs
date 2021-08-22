#[cfg(test)]
mod subscription {
    use redust::{Store, UnsubscribeError};

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
        fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
            1
        }

        let mut store = Store::new(reducer, 0);
        let subscription_token = store.subscribe(|state| {
            assert_eq!(*state, 1);
        });

        store.dispatch(MyAction::Increment);

        let _result = store.unsubscribe(subscription_token);

        store.dispatch(MyAction::Increment);
    }

    #[test]
    fn should_throw_an_error_when_unsubscribe_will_call_with_wrong_token() {
        type MyStore = u8;

        #[derive(Debug)]
        enum MyAction {
            Increment,
        }
        fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
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
