type UserId = String;
type GroupId = u32;

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: UserId,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub group_id: GroupId,
    pub name: String,
    pub description: String,
    pub created_by: UserId,
}

#[derive(Debug, Clone)]
pub struct GroupMembership {
    pub user_id: UserId,
    pub group_id: GroupId,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub added_by: UserId,
    pub group: GroupId,
    pub amount: u32,
    pub title: String,
    pub description: String,
}

impl User {
    pub fn new(user_id: UserId, name: String, username: String) -> User {
        User {
            user_id,
            name,
            username,
        }
    }
}

impl Group {
    pub fn new(group_id: GroupId, name: String, description: String, created_by: UserId) -> Group {
        Group {
            group_id,
            name,
            description,
            created_by,
        }
    }
}

impl GroupMembership {
    pub fn new(user_id: UserId, group_id: GroupId) -> GroupMembership {
        GroupMembership { user_id, group_id }
    }
}

impl Expense {
    pub fn new(
        added_by: UserId,
        group: GroupId,
        amount: u32,
        title: String,
        description: String,
    ) -> Expense {
        Expense {
            added_by,
            group,
            amount,
            title,
            description,
        }
    }
}
