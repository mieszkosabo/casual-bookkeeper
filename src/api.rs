use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

use crate::{group::{Group}, member::Member};

pub enum Operation {
    CreateGroup { name: String },
    AddMember { group_id: String, member_name: String },
    RemoveMember { group_id: String, member_name: String },
    RemoveGroup {group_id: String },
}

#[derive(Serialize, Deserialize)]
struct State {
    groups: HashMap<String, Group>,
}

impl State {
    fn new() -> State {
        State {
            groups: HashMap::new(),
        }
    }

    fn save(&self) -> Result<()> {
        let json = serde_json::to_string(self)?;
        std::fs::write("state.json", json)?;
        Ok(())
    }

    fn load() -> State {
        let json = std::fs::read_to_string("state.json").unwrap_or(String::from("{}"));
        serde_json::from_str(&json).unwrap_or(State::new())
    }

    fn erase() -> Result<()> {
        if std::fs::metadata("state.json").is_ok() {
            std::fs::remove_file("state.json")?;
        }
        Ok(())
    }
}

pub fn erase_state() -> Result<()> {
    State::erase()
}

pub fn create_group(name: String) -> Result<String> {
    let mut state = State::load();
    let group = Group::new(name);
    let id = group.id.clone();
    state.groups.insert(group.id.clone(), group);
    state.save()?;
    Ok(id)
}

pub fn remove_group(group_id: &String) -> Result<()> {
    let mut state = State::load();
    state.groups.remove(group_id);
    state.save()?;
    Ok(())
}

pub fn add_member(group_id: &String, member_name: String) -> Result<()> {
    let mut state = State::load();
    let group = state.groups.get_mut(group_id).context(format!("Group of id: {} was not found", group_id))?;
    if group.get_member(&member_name).is_some() {
        return Err(anyhow::anyhow!("Member with name: {} already exists in group", member_name));
    }
    let member = Member::new(member_name);
    group.add_member(member);
    state.save()?;
    Ok(())
}

pub fn remove_member(group_id: &String, member_name: String) -> Result<()> {
    let mut state = State::load();
    let group = state.groups.get_mut(group_id).context(format!("Group of id: {} was not found", group_id))?;
    group.remove_member(&member_name);
    state.save()?;
    Ok(())
}

