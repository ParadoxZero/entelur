pub type UserId = String;
pub type GroupId = u32;

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

#[derive(Debug, Clone, Copy)]
pub enum DataError {}

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

pub trait Datamodel {
    async fn add_user(&self, user: User) -> Result<(), DataError>;
    async fn add_group(&self, group: Group) -> Result<(), DataError>;
    async fn add_user_to_group(&self, group_id: GroupId, user_id: UserId) -> Result<(), DataError>;
    async fn add_expense(&self, expense: Expense) -> Result<(), DataError>;

    async fn get_user(&self, user_id: UserId) -> Result<User, DataError>;
    async fn get_group(&self, group_id: GroupId) -> Result<Group, DataError>;
    async fn get_group_members(&self, group_id: GroupId) -> Result<Vec<User>, DataError>;
    async fn get_expenses(&self, group_id: GroupId) -> Result<Vec<Expense>, DataError>;

    async fn remove_user_from_group(
        &self,
        group_id: GroupId,
        user_id: UserId,
    ) -> Result<(), DataError>;
    async fn delete_group(&self, group_id: GroupId) -> Result<(), DataError>;
    async fn delete_user(&self, user_id: UserId) -> Result<(), DataError>;
    async fn delete_expense(&self, expense_id: u32) -> Result<(), DataError>;

    async fn get_membership(&self, user_id: UserId) -> Result<GroupMembership, DataError>;
    async fn get_user_expenses(&self, user_id: UserId) -> Result<Vec<Expense>, DataError>;
}
