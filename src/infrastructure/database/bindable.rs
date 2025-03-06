#![allow(dead_code)]
use sqlx::{Sqlite, query::Query};
use chrono::NaiveDate;

pub trait Bindable {
    fn bind_value<'q>(
        &self,
        query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>;
}

pub trait CloneableBindable: Bindable {
    fn clone_box(&self) -> Box<dyn CloneableBindable + Send + Sync>;
}

macro_rules! impl_bindable {
    ($type:ty) => {
        impl CloneableBindable for $type {
            fn clone_box(&self) -> Box<dyn CloneableBindable + Send + Sync> {
                Box::new(*self)
            }
        }

        impl Bindable for $type {
            fn bind_value<'q>(
                &self,
                query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
            ) -> Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
                query.bind(*self)
            }
        }
    };
}

impl CloneableBindable for String {
    fn clone_box(&self) -> Box<dyn CloneableBindable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Bindable for String {
    fn bind_value<'q>(
        &self,
        query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
        query.bind(self.clone())
    }
}

impl CloneableBindable for u64 {
    fn clone_box(&self) -> Box<dyn CloneableBindable + Send + Sync> {
        Box::new(*self)
    }
}

impl Bindable for u64 {
    fn bind_value<'q>(
        &self,
        query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
        let value = if *self > i64::MAX as u64 {
            i64::MAX
        } else {
            *self as i64
        };
        query.bind(value)
    }
}

impl CloneableBindable for NaiveDate {
    fn clone_box(&self) -> Box<dyn CloneableBindable + Send + Sync> {
        Box::new(*self)
    }
}

impl Bindable for NaiveDate {
    fn bind_value<'q>(
        &self,
        query: Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    ) -> Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
        let date_str = self.format("%Y-%m-%d").to_string();
        query.bind(date_str)
    }
}

impl_bindable!(i32);
impl_bindable!(i64);
impl_bindable!(bool);
impl_bindable!(f32);
impl_bindable!(f64);
impl_bindable!(u8);
impl_bindable!(u16);
impl_bindable!(u32);
impl_bindable!(i8);
impl_bindable!(i16);