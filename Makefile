postgres-on-docker:
	docker run -d --name journaly_postgres -e POSTGRES_PASSWORD=postgres -p 5431:5432 postgres

redis-on-docker:
	docker run -d --name journaly_redis -p 6379:6379 redis/redis-stack-server:latest