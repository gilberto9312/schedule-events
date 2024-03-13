
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Events, NewEvent};
use chrono::{NaiveDate, NaiveTime};
use std::io::stdin;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_event(conn: &mut PgConnection, 
    title: String,
    date_start: NaiveDate,
    time_start: NaiveTime,
    time_end: NaiveTime,
    date_end: NaiveDate,
) -> Events {
    use crate::schema::events;

    let new_event = NewEvent { title: &title, date_start: &date_start, time_start: &time_start, time_end: &time_end, date_end: &date_end };

    diesel::insert_into(events::table)
        .values(&new_event)
        .returning(Events::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn list_event(conn: &mut PgConnection) {
    use self::schema::events::dsl::*;
    
    let results = events
        .limit(5)
        .select(Events::as_select())
        .load(conn)
        .expect("Error loading events");

    println!("Displaying {} events", results.len());
    for event in results {
        println!("{}", event.title);
        println!("-----------\n");
    }
}

pub mod models;
pub mod schema;
fn main() {
    let connection = &mut establish_connection();
    let mut option = String::new();
    let create = '1'.to_string();
    let list = '2'.to_string();

     println!("1: add event");
     println!("-----------\n");
     println!("2: list event");
     stdin().read_line(&mut option).unwrap();
     let option = option.trim_end(); 
     if option == create {
        let mut title = String::new();
        let mut date_start= String::new();
        let mut time_start= String::new();
        let mut time_end= String::new();
        let mut date_end= String::new();

        println!("What would you like your title to be?");
        stdin().read_line(&mut title).unwrap();
        let title = title.trim_end().to_string();
        println!("What would you like your date start to be?");
        stdin().read_line(&mut date_start).unwrap();
        let date_start =  NaiveDate::parse_from_str(date_start.trim_end(), "%Y-%m-%d").unwrap();
        println!("What would you like your time start to be?");
        stdin().read_line(&mut time_start).unwrap();
        let time_start = NaiveTime::parse_from_str(time_start.trim_end(), "%H:%M:%S").unwrap();
        println!("What would you like your date end to be?");
        stdin().read_line(&mut date_end).unwrap();
        let date_end =  NaiveDate::parse_from_str(date_end.trim_end(), "%Y-%m-%d").unwrap();
        println!("What would you like your time end to be?");
        stdin().read_line(&mut time_end).unwrap();
        let time_end = NaiveTime::parse_from_str(time_end.trim_end(), "%H:%M:%S").unwrap();

        create_event(connection,
            title,
            date_start,
            time_start,
            time_end,
            date_end,
        );
     }
     if option == list {
        list_event(connection);
     }
}
