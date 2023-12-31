-- Your SQL goes here
CREATE TABLE users (
  id UUID DEFAULT uuid_generate_v4() NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  phone_number VARCHAR,
  is_email_verified BOOLEAN DEFAULT FALSE NOT NULL,
  is_phone_number_verified BOOLEAN DEFAULT FALSE NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP,

  CONSTRAINT user_primary_key PRIMARY KEY (id)
)
