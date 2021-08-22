#[cfg(test)]
mod state {
    use redust::Store;

    #[test]
    fn should_return_initial_state_when_it_is_int_1() {
        type MyStore = u8;
        enum MyAction {}

        fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
            0
        }

        let store = Store::new(reducer, 10);

        assert_eq!(*store.state(), 10);
    }

    #[test]
    fn should_return_initial_state_when_it_is_f32_10_2() {
        type MyStore = f32;
        enum MyAction {}

        fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
            0.0
        }

        let store = Store::new(reducer, 10.2);

        assert_eq!(*store.state(), 10.2);
    }

    #[test]
    fn should_return_initial_state_when_it_is_vector_1_2_3() {
        type MyStore = Vec<u8>;
        enum MyAction {}

        fn reducer(_state: &MyStore, _action: &MyAction) -> MyStore {
            vec![10, 20, 30]
        }

        let store = Store::new(reducer, vec![1, 2, 3]);

        assert_eq!(*store.state(), vec![1, 2, 3]);
    }

    #[test]
    fn should_return_state_when_it_changed_from_the_reducer() {
        type MyStore = Vec<u8>;
        enum MyAction {
            // Add new value to the array
            Add(u8),
        }

        fn reducer(state: &MyStore, action: &MyAction) -> MyStore {
            match action {
                MyAction::Add(new_value) => {
                    let mut new_state = state.clone();
                    new_state.push(*new_value);

                    new_state
                }
            }
        }

        let mut store = Store::new(reducer, vec![1, 2, 3]);
        assert_eq!(*store.state(), vec![1, 2, 3]);

        store.dispatch(MyAction::Add(4));
        assert_eq!(*store.state(), vec![1, 2, 3, 4])
    }
}
