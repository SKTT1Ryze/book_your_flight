//! 前端 UI 界面之状态机实现(用于场景变换你)
//! 
//! 场景变换

use std::hash::Hash;
use std::collections::HashMap;

use crate::config::STATE_NUM;

/// 状态机
/// 
/// todo: 考虑锁机制的需求
pub struct StateMachine<I, const N: usize>
where
    I: Eq + Hash + Copy
{
    states: [State<I>; N],
    current_state_id: usize
}

impl<I: Eq + Hash + Copy> StateMachine<I, STATE_NUM> {
    pub fn unused() -> Self {
        let states = [
            State::default(), State::default(), State::default(), State::default(),
            State::default(), State::default(), State::default()
            ];
        Self {
            states,
            current_state_id: 0
        }
    }

    
}

impl StateMachine<usize, STATE_NUM> {
    pub fn init() -> Self {
        let mut state0 = State::new(0);
        state0.insert_next_state(1, 1);
        state0.insert_next_state(2, 2);
        let mut state1 = State::new(1);
        state1.insert_next_state(0, 0);
        state1.insert_next_state(2, 2);
        let mut state2 = State::new(2);
        state2.insert_next_state(0, 0);
        state2.insert_next_state(1, 1);
        state2.insert_next_state(4, 3);
        let mut state3 = State::new(3);
        state3.insert_next_state(0, 0);
        state3.insert_next_state(1, 1);
        state3.insert_next_state(3, 4);
        state3.insert_next_state(4, 5);
        state3.insert_next_state(5, 2);
        let mut state4 = State::new(4);
        state4.insert_next_state(0, 0);
        state4.insert_next_state(1, 1);
        state4.insert_next_state(3, 6);
        state4.insert_next_state(4, 3);
        let mut state5 = State::new(5);
        state5.insert_next_state(0, 0);
        state5.insert_next_state(1, 1);
        state5.insert_next_state(5, 3);
        let mut state6 = State::new(6);
        state6.insert_next_state(0, 0);
        state6.insert_next_state(1, 1);
        state6.insert_next_state(4, 4);
        let states = [
            state0, state1, state2, state3,
            state4, state5, state6
        ];
        Self {
            states,
            current_state_id: 0
        }
    }
}

impl<I, const N: usize> StateMachine<I, N>
where
    I: Eq + Hash + Copy
{
    pub fn current_state(&self) -> usize {
        self.current_state_id
    }

    /// 转移成功返回 Some(next_state_id)，状态不变返回 None
    pub fn state_transfer(&mut self, input: I) -> Option<usize> {
        let current_state = &self.states[self.current_state_id];
        current_state.next_state(&input).map(|s| {
            self.current_state_id = s;
            s
        })
    }

    /// 返回需要渲染的场景编号
    pub fn scene_id(&self) -> usize {
        self.states[self.current_state_id].scene_id
    }
}

/// 状态
#[derive(Clone)]
pub struct State<I>
where
    I: Eq + Hash + Copy
{
    /// 状态 ID
    pub id: usize,
    /// 哈希表保存各种输入对应的下一个状态
    pub next_states: HashMap<I, usize>,
    /// 该状态对应的场景 ID，用于下标检索
    pub scene_id: usize
}

impl<I: Eq + Hash + Copy> Default for State<I> {
    fn default() -> Self {
        Self {
            id: 0,
            next_states: HashMap::new(),
            scene_id: 0
        }
    }
}


impl<I> State<I>
where
    I: Eq + Hash + Copy
{
    pub fn new(id: usize) -> Self {
        Self {
            id,
            next_states: HashMap::new(),
            scene_id: id
        }
    }

    /// 插入新的输入和下一个状态的对应关系，如果已经存在则返回旧的值，否则返回 None
    pub fn insert_next_state(&mut self, input: I, next_state: usize) -> Option<usize> {
        self.next_states.insert(input, next_state)
    }

    /// 根据输入获得下一个状态的 ID，如果状态不变返回 None
    pub fn next_state(&self, input: &I) -> Option<usize> {
        self.next_states.get(input).map(|s| *s)
    }
}
