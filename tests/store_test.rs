macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[cfg(test)]
mod store {
    mod todos {
        use redust::Store;
        use std::collections::HashMap;

        type TodoId = u8;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Todo {
            id: TodoId,
            checked: bool,
            title: &'static str,
        }

        #[derive(Debug, Clone, PartialEq)]
        struct Todos {
            todos: HashMap<TodoId, Todo>,
        }

        impl Default for Todos {
            fn default() -> Self {
                Self {
                    todos: HashMap::new(),
                }
            }
        }

        enum TodosActions {
            // Adds new Todo into the `Todos`
            Add(Todo),

            // Removes Todo by id
            Remove(TodoId),

            // Changes Todo by id to the new `Todo`
            Change(TodoId, Todo),

            // Checks (set checked: true) Todo by id
            Check(TodoId),
        }

        fn todo_reducer(state: &Todos, action: &TodosActions) -> Todos {
            match action {
                TodosActions::Add(todo) => {
                    let mut new_state = state.clone();
                    new_state.todos.insert(todo.id, *todo);

                    new_state
                }
                TodosActions::Change(todo_id, todo) => {
                    let mut new_state = state.clone();
                    if let Some(found_todo) = new_state.todos.get_mut(todo_id) {
                        *found_todo = *todo;
                    }

                    new_state
                }
                TodosActions::Remove(todo_id) => {
                    let mut new_state = state.clone();
                    new_state.todos.remove(&todo_id);

                    new_state
                }
                TodosActions::Check(todo_id) => {
                    let mut new_state = state.clone();
                    let todo = new_state.todos.get_mut(todo_id);

                    if let Some(todo) = todo {
                        todo.checked = true;
                    }

                    new_state
                }
            }
        }

        #[test]
        fn should_mark_as_checked_todo3_when_dispatch_with_action_checked_was_called() {
            let todo1 = Todo {
                id: 1,
                title: "Todo 1",
                checked: false,
            };
            let todo2 = Todo {
                id: 2,
                title: "Todo 2",
                checked: false,
            };
            let todo3 = Todo {
                id: 3,
                title: "Todo 3",
                checked: false,
            };

            let initial_todos = hashmap![
                todo1.id => todo1,
                todo2.id => todo2,
                todo3.id => todo3
            ];

            let mut store = Store::new(
                todo_reducer,
                Todos {
                    todos: initial_todos,
                },
            );

            // Should be checked the `todo` number 3 as `checked: true`
            store.dispatch(TodosActions::Check(todo3.id));

            assert_eq!(
                store.state().todos.get(&todo3.id),
                Some(&Todo {
                    id: todo3.id,
                    title: todo3.title,
                    checked: true,
                })
            );
        }

        #[test]
        fn should_add_into_todos_new_todo_when_dispatch_with_action_add_was_called() {
            let todo1 = Todo {
                id: 1,
                title: "Todo 1",
                checked: false,
            };
            let todo2 = Todo {
                id: 2,
                title: "Todo 2",
                checked: false,
            };
            let todo3 = Todo {
                id: 3,
                title: "Todo 3",
                checked: false,
            };

            let initial_todos = hashmap![
                todo1.id => todo1,
                todo2.id => todo2,
                todo3.id => todo3
            ];

            let mut store = Store::new(
                todo_reducer,
                Todos {
                    todos: initial_todos,
                },
            );

            let todo4 = Todo {
                id: 4,
                title: "Todo 4",
                checked: false,
            };

            store.dispatch(TodosActions::Add(todo4));
            assert_eq!(store.state().todos.len(), 4);
        }

        #[test]
        fn should_change_todo_by_id_when_dispatch_with_action_change_was_called() {
            let todo1 = Todo {
                id: 1,
                title: "Todo 1",
                checked: false,
            };
            let todo2 = Todo {
                id: 2,
                title: "Todo 2",
                checked: false,
            };
            let todo3 = Todo {
                id: 3,
                title: "Todo 3",
                checked: false,
            };

            let initial_todos = hashmap![
                todo1.id => todo1,
                todo2.id => todo2,
                todo3.id => todo3
            ];

            let mut store = Store::new(
                todo_reducer,
                Todos {
                    todos: initial_todos,
                },
            );

            let updated_todo2 = Todo {
                id: todo2.id,
                title: "Updated Todo 2",
                checked: todo2.checked,
            };

            store.dispatch(TodosActions::Change(todo2.id, updated_todo2));
            assert_eq!(store.state().todos.get(&todo2.id), Some(&updated_todo2));
        }

        #[test]
        fn should_remove_todo_by_id_when_dispatch_with_action_remove_was_called() {
            let todo1 = Todo {
                id: 1,
                title: "Todo 1",
                checked: false,
            };
            let todo2 = Todo {
                id: 2,
                title: "Todo 2",
                checked: false,
            };
            let todo3 = Todo {
                id: 3,
                title: "Todo 3",
                checked: false,
            };

            let initial_todos = hashmap![
                todo1.id => todo1,
                todo2.id => todo2,
                todo3.id => todo3
            ];

            let mut store = Store::new(
                todo_reducer,
                Todos {
                    todos: initial_todos,
                },
            );

            store.dispatch(TodosActions::Remove(todo2.id));
            assert_eq!(store.state().todos.len(), 2);

            // We cannot get todo2 from the HashMap because we already removed it
            assert_eq!(store.state().todos.get(&todo2.id), None);
        }
    }
}
