# Local Setup
> This doc will explain how to setup Journaly locally.

### Note for Windows Users
To run Makefiles you will need to have `make` installed.

To install `make`, ensure that [Chocolatey](https://docs.chocolatey.org/en-us/choco/setup/#more-install-options) is also installed and that your systems Path environment variable points to its bin directory. 
Now run the following command:
```
choco install make
```

## Set up PostgreSQL
**Option 1** : Without docker.

To install PostgreSQL, follow the installation guide on the [official PostgreSQL website](https://www.postgresql.org/download/).

**Option 2** : With docker.
```
make postgres-on-docker
```

Postgres will now be running on `localhost:5432`.

You can access the environment through CLI by running:
```
docker exec -it journaly_postgres psql -U postgres
```


## Set up Redis
Redis can only be installed in a Linux environment. So just use docker.

If you have docker installed, run the following command:

```
make redis-on-docker
```

Redis will now be running on `localhost:6379`.
