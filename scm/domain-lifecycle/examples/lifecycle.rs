//! Basic `Lifecycle` usage example.

use edge_domain_lifecycle::{Lifecycle, LifecycleBootstrap, StdLifecycleFactory, TransitionPolicy};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TaskState {
    Pending,
    Running,
    Done,
    Failed,
}

/// Only allow transitions that move a task forward; never backward.
struct ForwardOnlyPolicy;

impl TransitionPolicy for ForwardOnlyPolicy {
    type State = TaskState;

    fn is_allowed(&self, from: TaskState, to: TaskState) -> bool {
        matches!(
            (from, to),
            (TaskState::Pending, TaskState::Running)
                | (TaskState::Running, TaskState::Done)
                | (TaskState::Running, TaskState::Failed)
        )
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
        .block_on(async {
            let task =
                StdLifecycleFactory::managed(TaskState::Pending, Box::new(ForwardOnlyPolicy));

            task.transition_to(TaskState::Running).await.expect("Pending→Running");
            println!("state = {:?}", task.state());

            task.transition_to(TaskState::Done).await.expect("Running→Done");
            println!("state = {:?}", task.state());

            // backward transition is rejected
            let result = task.transition_to(TaskState::Pending).await;
            println!("reverse transition: {:?}", result);
        });
}
