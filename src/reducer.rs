pub type Reducer<State, Action> = fn(&State, &Action) -> State;
