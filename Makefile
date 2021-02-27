#!make
include .env
export

watch:
	RUST_LOG=debug cargo watch -w src -x "lrun --bin main"
seed:
	-psql $$DATABASE_URL -c "\copy users from 'seed_data/users.csv' with csv header";
	-psql $$DATABASE_URL -c "\copy courses from 'seed_data/courses.csv' with csv header";
	-psql $$DATABASE_URL -c "\copy participations from 'seed_data/participations.csv' with csv header"