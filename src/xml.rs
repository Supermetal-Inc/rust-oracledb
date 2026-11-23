//-----------------------------------------------------------------------------
// Copyright (c) 2026, Oracle and/or its affiliates.
//
// This software is dual-licensed to you under the Universal Permissive License
// (UPL) 1.0 as shown at https://oss.oracle.com/licenses/upl and Apache License
// 2.0 as shown at http://www.apache.org/licenses/LICENSE-2.0. You may choose
// either license.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

use crate::db_value::{DbValue, FromDbValue};
use crate::row::ColumnData;
use crate::{Error, Lob};

/// A directly fetched XMLTYPE value. Take it from a row to retain a LOB locator.
/// Neither representation is parsed or reformatted by the driver.
pub enum Xml {
    /// XML supplied inline by the database.
    Text(String),
    /// XML supplied through a character LOB locator.
    Lob(Lob),
}

impl<'a> FromDbValue<'a> for Xml {
    fn from_db_value(data: ColumnData<'a>) -> Result<Self, Error> {
        match data {
            ColumnData::Owned(Some(DbValue::String(value))) => {
                Ok(Self::Text(value))
            }
            ColumnData::Borrowed(Some(DbValue::String(value))) => {
                Ok(Self::Text(value.clone()))
            }
            ColumnData::Owned(Some(DbValue::Lob(value)))
                if value.db_type() == crate::DB_TYPE_CLOB =>
            {
                Ok(Self::Lob(value))
            }
            _ => Err(Self::unsupported_conversion(data)),
        }
    }
}
