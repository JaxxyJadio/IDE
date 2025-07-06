use std::collections::VecDeque;

/// Represents a single instruction for the agent.
#[derive(Debug, Clone)]
pub struct AgentInstruction {
    pub text: String,
    pub timestamp: std::time::SystemTime,
}

/// Manages a queue of instructions for the agent.
pub struct AgentInstructionQueue {
    queue: VecDeque<AgentInstruction>,
}

impl AgentInstructionQueue {
    /// Create a new, empty instruction queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Add a new instruction to the queue.
    pub fn add_instruction(&mut self, text: impl Into<String>) {
        let instruction = AgentInstruction {
            text: text.into(),
            timestamp: std::time::SystemTime::now(),
        };
        self.queue.push_back(instruction);
    }

    /// Get the next instruction, if any (FIFO).
    pub fn next_instruction(&mut self) -> Option<AgentInstruction> {
        self.queue.pop_front()
    }

    /// Peek at the next instruction without removing it.
    pub fn peek_instruction(&self) -> Option<&AgentInstruction> {
        self.queue.front()
    }

    /// Clear all instructions.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Get the number of instructions in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
