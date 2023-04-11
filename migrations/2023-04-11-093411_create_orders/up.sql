-- Your SQL goes here
create table if not exists orders (
	id serial primary key,
	customer_id serial references customers(id),
	status text not null,
	created_at timestamp default now()
);