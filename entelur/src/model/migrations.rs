/*
This file is part of Entelur (https://github.com/ParadoxZero/entelur/).
Copyright (c) 2024 Sidhin S Thomas.

Entelur is free software: you can redistribute it and/or modify it under the terms of the 
GNU General Public License as published by the Free Software Foundation, either version 3 
of the License, or (at your option) any later version.

Entelur is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. 
See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Foobar. 
If not, see <https://www.gnu.org/licenses/>.
*/

use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Copy)]
pub(super) struct MigrationData {
    pub(super) version: u32,
    pub(super) last_migration_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Migration{
    pub(super) version: u32,
    pub(super) sql_statements: &'static str
}