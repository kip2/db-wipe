<h1 align="center"> db-wipe </h1>

<h2 align="center"> Streamline Your Database Management </h2>

`db-wipe` is a console application that simplifies database management, offering tools for data deletion and restoration.

It efficiently clears or restores your database with minimal hassle.

---


## Features

- **Effortless Data Deletion**: Removing data from your database is a breeze with `db-wipe`. For added convenience and safety, you can use the `-d` option to create a backup before wiping the data, but this step is optional.

- **Seamless Restoration**: Easily restore your database with `db-wipe -r`, ensuring you have the `database-name.bk.sql` dump file ready.

### MySQL Support

`db-wipe` is specifically designed for use with MySQL databases, utilizing `mysqldump` for creating backups before data deletion. 

This ensures a safe and efficient way to manage your MySQL database tasks.

- **MySQL Compatible**: `db-wipe` is tailor-made for managing MySQL databases. 

It harnesses the power of `mysqldump` to back up your data safely before any deletion occurs.

To use `db-wipe` with a MySQL database, ensure you have `mysqldump` installed and accessible in your system's PATH. 

This will allow `db-wipe` to automatically create backups when using the `-d` option.

## Configuration

Before using `db-wipe`, you need to set up the necessary configuration in a `.env` file at the root of your project. 

This file should contain the following entries:

```env
USER_NAME=user_name    # Your database username
PASSWORD=password      # Your database password
DB_NAME=db_name        # The name of the database to manage
PORT=port_number       # The port number your database server is listening on
HOST=host_name         # The hostname or IP address of your database server
```

Ensure that each placeholder (e.g., user_name, password, etc.) is replaced with your actual database credentials and connection details.

This setup allows db-wipe to connect to and manage your database correctly.

### Usage

To delete your database:
```
db-wipe
```

If no options are provided, you will be prompted to choose whether to create a dump of the database contents.

To create a dump before deletion:
```
db-wipe -d
```

To restore your database:
```
db-wipe -r
```

Ensure you have the dump file named `database-name.bk.sql` in the current directory for restoration.

## Installation

Binary distributions are not provided, so you will need to build the application in your own environment. Follow these steps to build:

1. Clone the repository:

```bash
git clone <repository-url>
```

2. Move to the project's root path:

```bash
cd sqcr
```

3. Build the project using `cargo build`:

```bash
cargo build --release
```

This command compiles the project in release mode, creating an optimized executable binary.

After building, the executable file can be found in the target/release directory within your project folder.

---

## AUTHOR

[kip2](https://github.com/kip2)
