
use std::collections::HashMap;
use std::fmt;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::calendar::calendar::Calendar;
use crate::calendar::skyblock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id:Uuid,
    name: String,
    calendars: HashMap<Uuid, Calendar>,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let calendar_list = self.list_calendars()
            .into_iter().map(|user| format!("{}", user))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "id: {} name: {} calendars: [{}]",self.id, self.name, calendar_list)
    }
}
impl User {
    pub fn new(name: String) -> Self {
        User { id: Uuid::new_v4(), name, calendars: HashMap::new() }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_calendar(&mut self, calendar: Calendar) {
        self.calendars.insert(Uuid::new_v4(), calendar);
    }

    pub fn get_calendar(&self, id: &Uuid) -> Option<&Calendar> {
        self.calendars.get(&id)
    }
    pub fn list_calendars(&self) -> Vec<&Calendar> {
        self.calendars.values().collect::<Vec<&Calendar>>()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataBase {
    users: HashMap<Uuid, User>
}
impl DataBase {
    pub fn new() -> Self {
        DataBase { users: HashMap::new() }.init()
    }
    fn init(mut self) -> Self {
        let mut global_user = User::new("GLOBAL".to_string());
        let skyblock = skyblock::generate_calendar(Utc::now(), Utc::now() + Duration::minutes(7460));
        global_user.add_calendar(skyblock);
        self.add_user(global_user);
        self
    }
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    pub fn get_user(&self, user_id:Uuid) -> Option<&User> {
        self.users.get(&user_id)
    }
    pub fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
}

impl fmt::Display for DataBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user_list = self
            .list_users()
            .iter()
            .map(|user| format!("{}", user))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "Database with users: [{}]", user_list)
    }
}
