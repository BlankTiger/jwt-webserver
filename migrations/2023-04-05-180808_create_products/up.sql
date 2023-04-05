-- create products table in sqlite3, id, name, price, available
create table products (
	id integer not null primary key autoincrement,
	name text not null,
	price real not null,
	available boolean not null
);
