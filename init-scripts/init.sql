-- init.sql

CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- Para garantir que a extensão UUID seja carregada, caso necessário

CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY,  -- UUID será passado manualmente
                                     email VARCHAR(255) NOT NULL,
    password TEXT NOT NULL,
    nickname VARCHAR(50),
    created_at TIMESTAMP
    );
