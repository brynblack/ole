CREATE TABLE lessons (
  id SERIAL PRIMARY KEY,
  reference VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  image VARCHAR NOT NULL
)