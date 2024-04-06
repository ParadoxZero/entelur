
type UserId=String;
type GroupId=u32;
pub struct User {
    pub user_id: UserId,
    pub name: String,
    pub username: String
}

pub struct Group {
    pub group_id: GroupId,
    pub name: String,
    pub description: String,
    pub created_by: UserId
}

pub struct GroupMembership {
    pub user_id: UserId,
    pub group_id: GroupId
}

pub struct Expense {
    pub added_by: UserId,
    pub group: Option<GroupId>,
    pub amount: u32,
    pub title: String,
    pub description:String
}