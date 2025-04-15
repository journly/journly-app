# Local Setup
> This doc will explain how to setup Journaly locally.

To begin local setup, clone the repository from github.
```
git clone https://github.com/journaly-app/journaly.git <OPTIONAL DIRECTORY>
```

### Easiest setup
For the easiest setup, ensure that Docker is installed onto your system.

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

Alternatively (visual solution), you can connect to the Postgres server using PgAdmin:
1. Open PgAdmin
2. Right-click the **Servers** drop-down tab and click **register** > **server**
3. Name the server anything (e.g. Journaly Postgres)
4. In the connections tab, fill in the following inputs with the corresponding values:
   ```
   Host name/address: localhost
   Port: 5431
   Maintenance database: postgres
   Username: postgres
   Password: postgres
   ```
5. Any remaining inputs can be left unchanged
6. Now, hit save and a connection should be established!

**IMPORTANT:** Upon connecting to the Postgres Server, create a new database called `test`, so that the backend server and perform migrations.

## Set up Redis
Redis can only be installed in a Linux environment. So just use docker.

If you have docker installed, run the following command:

```
make redis-on-docker
```

Redis will now be running on `localhost:6379`.
