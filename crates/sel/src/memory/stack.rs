use crate::{Result, VMError, Value};
use std::collections::HashMap;

pub struct Stack {
    pub items: Vec<Value>,
    limit: usize,
}

impl Stack {
    pub fn new(limit: usize) -> Self {
        Self {
            items: Vec::with_capacity(limit),
            limit,
        }
    }

    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.items.len() >= self.limit {
            return Err(VMError::StackOverflow);
        }
        self.items.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.items.pop().ok_or(VMError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<&Value> {
        self.items.last().ok_or(VMError::StackUnderflow)
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn swap(&mut self) -> Result<()> {
        if self.items.len() < 2 {
            return Err(VMError::StackUnderflow);
        }
        let len = self.items.len();
        self.items.swap(len - 1, len - 2);
        Ok(())
    }

    pub fn dup(&mut self) -> Result<()> {
        let value = self.peek()?.clone();
        self.push(value)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

pub struct Memory {
    pub stack: Stack,
    variables: HashMap<usize, Value>,
    call_stack: Vec<usize>,
}

impl Memory {
    pub fn new(stack_size: usize) -> Self {
        Self {
            stack: Stack::new(stack_size),
            variables: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    // Stack operations
    pub fn push(&mut self, value: Value) -> Result<()> {
        self.stack.push(value)
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Result<&Value> {
        self.stack.peek()
    }

    pub fn stack_size(&self) -> usize {
        self.stack.size()
    }

    pub fn swap(&mut self) -> Result<()> {
        self.stack.swap()
    }

    pub fn dup(&mut self) -> Result<()> {
        self.stack.dup()
    }

    // Variable operations
    pub fn store_var(&mut self, id: usize, value: Value) {
        self.variables.insert(id, value);
    }

    pub fn load_var(&self, id: usize) -> Result<Value> {
        self.variables
            .get(&id)
            .cloned()
            .ok_or(VMError::UndefinedVariable(id))
    }

    // Call stack operations
    pub fn push_call(&mut self, return_addr: usize) {
        self.call_stack.push(return_addr);
    }

    pub fn pop_call(&mut self) -> Result<usize> {
        self.call_stack
            .pop()
            .ok_or(VMError::RuntimeError("Return without call".to_string()))
    }

    // Stack snapshot para debugging
    pub fn get_stack_snapshot(&self) -> Vec<Value> {
        self.stack.items.clone()
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.variables.clear();
        self.call_stack.clear();
    }
}
