use redis::Client;
use std::marker::PhantomData;

pub mod user_dao;

pub mod trip_dao;

pub struct Data<T> {
    pub pg_pool: deadpool_postgres::Pool,
    pub redis_pool: r2d2::Pool<Client>,
    _marker: PhantomData<T>,
}

impl<T> Data<T> {
    pub fn new(pg_pool: deadpool_postgres::Pool, redis_pool: r2d2::Pool<Client>) -> Self {
        Self {
            pg_pool,
            redis_pool,
            _marker: PhantomData,
        }
    }
}
