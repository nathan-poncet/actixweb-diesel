CREATE TABLE reservations (
    table_id INTEGER NOT NULL REFERENCES tables (id),
    customer_id INTEGER NOT NULL REFERENCES customers (id),
    reservation_date DATE NOT NULL,
    party_size INTEGER NOT NULL,
    Primary Key (table_id, customer_id)
);