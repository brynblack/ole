CREATE TABLE courses (
  id SERIAL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  image VARCHAR NOT NULL
)