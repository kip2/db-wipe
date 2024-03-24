<h1 align="center"> db-wipe </h1>

<h2 align="center"> Streamline Your Database Management </h2>

`db-wipe` is an intuitive console application that simplifies database management by providing robust tools for data deletion and restoration.

It's built with efficiency in mind, allowing you to quickly clear your database or restore it from a backup with minimal hassle.

### Features

- **Effortless Data Deletion**: With `db-wipe`, removing data from your database is a breeze. For added convenience and safety, use the `-d` option to create a backup before wiping the data.

- **Seamless Restoration Process**: Restore your database effortlessly with `db-wipe -r`. Just ensure you have the appropriate dump file (`database-name.bk.sql`) ready, and let db-wipe handle the rest.

`db-wipe` is designed to make database management straightforward, providing peace of mind through its simple yet powerful functionality.

### Usage

To delete your database:
```
db-wipe
```

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

### AUTHOR

[kip2](https://github.com/kip2)
