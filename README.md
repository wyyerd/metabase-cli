# metabase-cli
A command-line interface for interacting with Metabase (primarily import/export).

## Usage
```bash
# Configure database credentials
> echo "DATABASE_URL=postgres://blah@blah/blah" > .env

# Export a dashboard/question (by id)
> metabase-cli export question 17 > question-17.json

# Import a dashboard/question (from a file)
> metabase-cli import question-17.json
```
