extern crate diesel;
extern crate chrono;

use self::diesel::prelude::*;
use chrono::{NaiveDate, NaiveTime};
use crate::schema::events;


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Events {
    pub id: i32,
    pub title: String,
    pub date_start: NaiveDate,
    pub time_start: NaiveTime,
    pub time_end: NaiveTime,
    pub date_end: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub title: &'a String,
    pub date_start: &'a NaiveDate,
    pub time_start: &'a NaiveTime,
    pub time_end: &'a NaiveTime,
    pub date_end: &'a NaiveDate,
}

