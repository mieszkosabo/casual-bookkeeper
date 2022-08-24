use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::member::Member;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub members: HashMap<String, Member>
}

impl Group {
    pub fn new(name: String) -> Group {
        Group {
            id: Uuid::new_v4().to_string(),
            name,
            members: HashMap::new()
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.insert(member.id.clone(), member);
    }

    pub fn remove_member(&mut self, member_name: &String) {
        self.members.retain(|_, member| &member.name != member_name);
    }

    pub fn get_member(&self, member_name: &String) -> Option<&Member> {
        self.members.values().find(|member| &member.name == member_name)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::member::Member;
    #[test]
    fn test_group_new() {
        let group = Group::new(String::from("test"));
        assert_eq!(group.name, String::from("test"));
        assert_eq!(group.members.len(), 0);
    }
    #[test]
    fn test_group_add_member() {
        let mut group = Group::new(String::from("test_group"));
        let member = Member::new(String::from("test_member"));
        group.add_member(member);
        assert_eq!(group.members.len(), 1);
    }
    #[test]
    fn test_group_remove_member() {
        let mut group = Group::new(String::from("test_group"));
        let member = Member::new(String::from("test_member"));
        let id = member.id.clone();
        group.add_member(member);
        assert_eq!(group.members.len(), 1);
        group.remove_member(&id);
        assert_eq!(group.members.len(), 0);
    }
}