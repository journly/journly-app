use std::marker::PhantomData;

pub mod user_dao;

pub mod trip_dao;

pub struct Data<T> {
    pub pg_pool: deadpool_postgres::Pool,
    _marker: PhantomData<T>,
}

impl<T> Data<T> {
    pub fn new(pg_pool: deadpool_postgres::Pool) -> Self {
        Self {
            pg_pool,
            _marker: PhantomData,
        }
    }
}
