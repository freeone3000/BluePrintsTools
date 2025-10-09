/// Performs a bounded depth-first search (DFS) to find a sequence of actions leading to a goal state.
/// /// # Arguments
/// * `initial_state` - The starting state for the search.
/// * `enumerate_actions_fn` - A function that takes a state and returns a vector of possible (useful!) actions
/// * `apply_action_fn` - A function that takes a state and an action, and returns the resulting state after applying the action.
/// * `is_goal_fn` - A function that takes a state and returns true if it is a goal state.
/// * `cur_actions` - A vector of actions taken to reach the current state (should be empty when first called).
/// * `max_depth` - The maximum depth to search.
/// # Returns
/// A tuple containing an optional vector of actions leading to the goal state (if found) and a boolean indicating if the search can potentially ever result in a solution.
pub fn bounded_dfs<StateType, ActionType>(
    initial_state: &StateType,
    enumerate_actions_fn: fn(&StateType) -> Vec<ActionType>,
    apply_action_fn: fn(&StateType, &ActionType) -> StateType,
    is_goal_fn: fn(&StateType) -> bool,
    is_solvable_fn: fn(&StateType) -> bool,
    cur_actions: Vec<ActionType>,
    max_depth: usize,
) -> (Option<Vec<ActionType>>, bool)
where
    StateType: Clone,
    ActionType: Clone,
{
    if is_goal_fn(initial_state) {
        (Some(cur_actions), true)
    } else if max_depth == 0 {
        (None, true)
    } else if !is_solvable_fn(initial_state) {
        (None, false)
    } else {
        let mut any_remaining = false;
        for action in enumerate_actions_fn(initial_state) {
            let new_state = apply_action_fn(initial_state, &action);
            let mut new_actions = cur_actions.clone();
            new_actions.push(action);
            let (result, remaining) = bounded_dfs(
                &new_state,
                enumerate_actions_fn,
                apply_action_fn,
                is_goal_fn,
                is_solvable_fn,
                new_actions,
                max_depth - 1,
            );
            if result.is_some() {
                return (result, true);
            }
            if remaining {
                any_remaining = true;
            }
        }
        (None, any_remaining)
    }
}

pub fn bfs<StateType, ActionType>(
    initial_state: &StateType,
    enumerate_actions_fn: fn(&StateType) -> Vec<ActionType>,
    apply_action_fn: fn(&StateType, &ActionType) -> StateType,
    is_goal_fn: fn(&StateType) -> bool,
    is_solvable: fn(&StateType) -> bool,
) -> Option<Vec<ActionType>>
where
    StateType: Clone + Eq + std::hash::Hash,
    ActionType: Clone,
{
    use std::collections::{HashSet, VecDeque};

    let mut queue: VecDeque<(StateType, Vec<ActionType>)> = VecDeque::new();
    let mut visited: HashSet<StateType> = HashSet::new();

    queue.push_back((initial_state.clone(), vec![]));
    visited.insert(initial_state.clone());

    while let Some((current_state, actions)) = queue.pop_front() {
        if is_goal_fn(&current_state) {
            return Some(actions);
        }

        for action in enumerate_actions_fn(&current_state) {
            let new_state = apply_action_fn(&current_state, &action);
            if !visited.contains(&new_state) && is_solvable(&new_state) {
                let mut new_actions = actions.clone();
                new_actions.push(action);
                queue.push_back((new_state.clone(), new_actions));
                visited.insert(new_state);
            }
        }
    }

    None
}