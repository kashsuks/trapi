CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE workouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    workout_type TEXT NOT NULL,
    distance_km DOUBLE PRECISION,
    duration_seconds INTEGER,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT workouts_type_check
        CHECK (workout_type IN ('run', 'bike', 'swim', 'hike', 'lift', 'row')),
    CONSTRAINT workouts_distance_nonnegative_check
        CHECK (distance_km IS NULL OR distance_km >= 0),
    CONSTRAINT workouts_duration_nonnegative_check
        CHECK (duration_seconds IS NULL OR duration_seconds >= 0)
);

CREATE INDEX idx_workouts_user_id ON workouts(user_id);
CREATE INDEX idx_workouts_type ON workouts(workout_type);
CREATE INDEX idx_workouts_created_at ON workouts(created_at);
CREATE INDEX idx_workouts_user_created_at ON workouts(user_id, created_at DESC);

