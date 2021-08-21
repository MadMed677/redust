#[cfg(test)]
mod dispatch {
    use redust::Store;

    #[test]
    fn dispatch_should_cause_updates_in_the_store() {
        type MyStore = u8;

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
        assert_eq!(*store.state(), 1);

        // Should increase the value to 2
        store.dispatch(MyAction::Increment);
        assert_eq!(*store.state(), 2);

        // Should decrease the value to 1
        store.dispatch(MyAction::Decrement);
        assert_eq!(*store.state(), 1);
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
        assert_eq!(*store.state(), 5);

        // Should decrement the value to 3
        store.dispatch(MyAction::DecrementBy(2));
        assert_eq!(*store.state(), 3);

        // Should decrement the value to 4
        store.dispatch(MyAction::IncrementBy(1));
        assert_eq!(*store.state(), 4);
    }

    #[test]
    fn dispatch_should_called_multiple_times_when_chain() {
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

        // Should increase the value to 3
        store
            .dispatch(MyAction::Increment)
            .dispatch(MyAction::Increment)
            .dispatch(MyAction::Increment);
        assert_eq!(*store.state(), 3);
    }
}
