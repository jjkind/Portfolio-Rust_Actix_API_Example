# rust-actix-api-example

Example project for running a RESTful API that leverages Rust/Actix Server connected to Postgres SQL DB (running in a container)

## Local Development Process

Running docker-compose.yml file will create a container with your REST API.  You will need to run the docker-compose.test.yml to create a Postgres SQL DB for testing

### Get your Postgres DB up and running

First run your docker-compose.test.yml file with the following command:

```
docker-compose -f docker-compose.test.yml up -d
```

Note we need the -d argument to run in detached mode.  Now check to see that your db container was created successfully and is running:
```
docker ps -a
```
Now you'll want to remote into your container and connect to your db via psql:

```
docker exec -it <container id> psql -h localhost -U postgres
```
If you need to remove all of your existing containers the following is a useful command:

```
docker rm $(docker ps -aq)
```

### Install Diesel ORM/Query Builder for Rust

We will utilize Diesel to map objects between Rust and Postgres and Build Queries.  You will want to install Diesel locally for testing, but it will be integrated into our API Service container through its inclusion in our docker-compose.yml

The following is used to install Diesel in your local Linux environment:

```
cargo install diesel_cli --no-default-features --features postgres
```

Once Diesel installs you'll likely get a warning that you have to update your Linux PATH to use the installed binaries.  Run this command:

```
export PATH="/home/<user>/.cargo/bin:$PATH"
```
Note that your path will be specific to your machine.  Also note that you'll have to change directories back to your project directory where your Cargo.toml file resides to run the `diesel setup` command.  

```
cd /
cd mnt/c/<path to your project>
```
