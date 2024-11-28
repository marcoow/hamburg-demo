CREATE TABLE tasks (
  id uuid PRIMARY KEY default gen_random_uuid(),
  description varchar(255) NOT NULL
);
