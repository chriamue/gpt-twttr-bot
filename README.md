# gpt-twttr-bot
Twitter Bot tweeting generated text based on tweets of accounts it follows.
The text will be generated using the gpt-j model.

## quickstart

You need a twitter account and a developer project to generate Access Tokens and API Keys.
Copy example.env file to .env and edit the environments.
The user_name should be the twitter @username to prevent the bot to reply to itself.

```sh
docker-compose up
```
