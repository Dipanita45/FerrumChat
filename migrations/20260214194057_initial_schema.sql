-- Add migration script here
-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =====================
-- USERS
-- =====================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =====================
-- CHATS
-- =====================
CREATE TABLE IF NOT EXISTS chats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user1_id UUID NOT NULL,
    user2_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_chat_user1
        FOREIGN KEY (user1_id) REFERENCES users(id) ON DELETE CASCADE,

    CONSTRAINT fk_chat_user2
        FOREIGN KEY (user2_id) REFERENCES users(id) ON DELETE CASCADE,

    -- Prevent duplicate 1-to-1 chats
    CONSTRAINT unique_chat_pair UNIQUE (user1_id, user2_id)
);

-- =====================
-- MESSAGES
-- =====================
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    chat_id UUID NOT NULL,
    sender_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_message_chat
        FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE,

    CONSTRAINT fk_message_sender
        FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE
);

-- =====================
-- INDEXES (IMPORTANT)
-- =====================
CREATE INDEX IF NOT EXISTS idx_chats_user1 ON chats(user1_id);
CREATE INDEX IF NOT EXISTS idx_chats_user2 ON chats(user2_id);

CREATE INDEX IF NOT EXISTS idx_messages_chat ON messages(chat_id);
CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);
