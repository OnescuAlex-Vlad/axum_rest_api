CREATE TABLE IF NOT EXISTS users (
  id           SERIAL PRIMARY KEY,
  username     VARCHAR(64) NOT NULL UNIQUE,
  password     VARCHAR(64) NOT NULL,
  email        VARCHAR(64) NOT NULL,
  car_details  VARCHAR(255),
  bank_details VARCHAR(255),
  token        TEXT DEFAULT NULL
);


INSERT INTO users (username, password, email) VALUES ('deleteduser', '$2b$12$x3hs5oMgjHdcV1GUEElfsO19JtS6.ixJAX9Cj62GyhpdPAIW25sky', 'test@test.com');
