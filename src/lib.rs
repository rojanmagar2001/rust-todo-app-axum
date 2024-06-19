pub mod infrastructure {
    pub mod database;
}

pub mod user {
    pub mod handlers;
    pub mod models;
    pub mod repository;
    pub mod service;
    pub mod views;
    pub mod routes;
}

pub mod common {
    pub mod error;
    pub mod middleware;
    pub mod pagination;
    pub mod password_encoder;
}

pub mod config;
