# Introduction

api is a Rust project that implements an AWS Lambda function in Rust.

## Neon Postgres setup

This service is prepared to read Neon connection strings from environment variables.

- `DATABASE_URL`: the pooled Neon connection string used by the running Lambda
- `MIGRATION_DATABASE_URL`: the direct Neon connection string used for schema setup and migrations

Copy `.env.example` to `.env` and replace the placeholder values with the connection details from the Neon Console.

Example:

```env
DATABASE_URL=postgresql://app_user:replace_me@ep-example-pooler.ap-southeast-1.aws.neon.tech/expense_tracker?sslmode=require&channel_binding=require
MIGRATION_DATABASE_URL=postgresql://app_user:replace_me@ep-example.ap-southeast-1.aws.neon.tech/expense_tracker?sslmode=require&channel_binding=require
```

Why two URLs:

- Neon recommends pooled connections for serverless app traffic
- direct connections are safer for migrations and admin tasks

## Migrations

This project uses ordered SQL migration files in `migrations/`.

Examples:

- `001_init.sql`
- `002_add_transaction_notes.sql`
- `003_create_budget_table.sql`

Use the Rust migration runner to apply all unapplied files in order:

```bash
cargo run --bin migrate
```

The runner:

- scans the `migrations/` directory
- applies `.sql` files in filename order
- records applied files in a `schema_migrations` table
- skips files that were already applied

If you want to inspect or apply a single file manually with `psql`:

```bash
psql "$MIGRATION_DATABASE_URL" -f migrations/001_init.sql
```

Best practice:

- do not keep editing old migrations once they have been applied anywhere important
- create a new numbered SQL file for each schema change

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example apigw-request
```

For generic events, where you define the event data structure, you can create a JSON file with the data you want to test with. For example:

```json
{
    "command": "test"
}
```

Then, run `cargo lambda invoke --data-file ./data.json` to invoke the function with the data in `data.json`.

For HTTP events, you can also call the function directly with cURL or any other HTTP client. For example:

```bash
curl https://localhost:9000
```

Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying

To deploy the project, run `cargo lambda deploy`. This will create an IAM role and a Lambda function in your AWS account.

Read more about deploying your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/deploy.html).
