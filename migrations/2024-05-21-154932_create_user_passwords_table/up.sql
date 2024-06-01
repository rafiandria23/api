-- Your SQL goes here
CREATE TABLE user_passwords (
  id UUID DEFAULT uuid_generate_v4() NOT NULL,
  user_id UUID NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP,

  CONSTRAINT user_password_primary_key PRIMARY KEY (id),
  CONSTRAINT fk_users_user_passwords FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
