-- Create timestamp trigger function
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    cognito_id VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger for users
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();

-- Create travel entries table
CREATE TABLE IF NOT EXISTS travel_entries (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255) NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    visit_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_travel_entries_user_id ON travel_entries(user_id);
CREATE INDEX IF NOT EXISTS idx_travel_entries_visit_date ON travel_entries(visit_date);

-- Create travel entry images table
CREATE TABLE IF NOT EXISTS travel_entry_images (
    id SERIAL PRIMARY KEY,
    travel_entry_id INTEGER NOT NULL REFERENCES travel_entries(id) ON DELETE CASCADE,
    image_key VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for travel entry images
CREATE INDEX IF NOT EXISTS idx_travel_entry_images_entry_id ON travel_entry_images(travel_entry_id);

-- Create trigger for travel entry images
CREATE TRIGGER set_timestamp_travel_entry_images
    BEFORE UPDATE ON travel_entry_images
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp();