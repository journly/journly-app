use redis::Client;
use std::marker::PhantomData;

pub mod user_dao;

pub mod trip_dao;

pub struct Table<T> {
    pub pg_pool: deadpool_postgres::Pool,
    pub redis_pool: r2d2::Pool<Client>,
    _marker: PhantomData<T>,
}

impl<T> Table<T> {
    pub fn new(pg_pool: deadpool_postgres::Pool, redis_pool: r2d2::Pool<Client>) -> Self {
        Self {
            pg_pool,
            redis_pool,
            _marker: PhantomData,
        }
    }
}

pub struct JoinTable<T, F> {
    pub pg_pool: deadpool_postgres::Pool,
    pub redis_pool: r2d2::Pool<Client>,
    _marker_1: PhantomData<T>,
    _marker_2: PhantomData<F>,
}

impl<T, F> JoinTable<T, F> {
    pub fn new(pg_pool: deadpool_postgres::Pool, redis_pool: r2d2::Pool<Client>) -> Self {
        Self {
            pg_pool,
            redis_pool,
            _marker_1: PhantomData,
            _marker_2: PhantomData,
        }
    }
}
