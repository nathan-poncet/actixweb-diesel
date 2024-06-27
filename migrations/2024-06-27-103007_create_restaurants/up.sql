CREATE TYPE cuisine_type AS ENUM ('american', 'mexican', 'italian', 'chinese', 'japanese', 'indian', 'french', 'other');

CREATE TABLE restaurants (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    cuisine_type cuisine_type NOT NULL
);