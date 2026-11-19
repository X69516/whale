
use std::fmt;

/// Runtime lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    Initializing,
    Ready,
    ShuttingDown,
    Stopped,
    Failed,
}

impl LifecycleState {
    /// Checks whether a transition to another state is allowed.
    pub const fn can_transition_to(self, next: Self) -> bool {
        use LifecycleState::*;

        matches!(
            (self, next),
            (Initializing, Ready)
                | (Initializing, Failed)
                | (Initializing, ShuttingDown)
                | (Ready, ShuttingDown)
                | (Ready, Failed)
                | (ShuttingDown, Stopped)
                | (ShuttingDown, Failed)
        )
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped | Self::Failed)
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::ShuttingDown => "shutting_down",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        };

        f.write_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_valid_transitions() {
        assert!(
            LifecycleState::Initializing
                .can_transition_to(LifecycleState::Ready)
        );

        assert!(
            LifecycleState::Ready
                .can_transition_to(LifecycleState::ShuttingDown)
        );

        assert!(
            LifecycleState::ShuttingDown
                .can_transition_to(LifecycleState::Stopped)
        );
    }

    #[test]
    fn rejects_invalid_transitions() {
        assert!(
            !LifecycleState::Stopped
                .can_transition_to(LifecycleState::Ready)
        );

        assert!(
            !LifecycleState::Failed
                .can_transition_to(LifecycleState::Ready)
        );

        assert!(
            !LifecycleState::Initializing
                .can_transition_to(LifecycleState::Stopped)
        );
    }

    #[test]
    fn terminal_states_are_identified() {
        assert!(LifecycleState::Stopped.is_terminal());
        assert!(LifecycleState::Failed.is_terminal());
        assert!(!LifecycleState::Ready.is_terminal());
    }
}

