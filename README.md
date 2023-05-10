# Reborn ![Stars](https://img.shields.io/github/stars/realTristan/Reborn?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/Reborn?label=Watchers)
![banner](https://github.com/realTristan/Reborn/assets/75189508/0be2cf20-d63d-425a-ac96-0b1907896b4d)

# Program Preview
![Capture](https://user-images.githubusercontent.com/75189508/210023258-362d4ca1-9f21-4d33-b9a5-579b0796bc58.PNG)
![Capture2](https://user-images.githubusercontent.com/75189508/210025481-cddd53e1-99ae-4f29-a723-ac355593c81d.PNG)

# Discord Bot
## Commands
- /token create
- /token delete {token}
- /token info {token}

# API Documentation
## Accounts
### Register with HWID
```go
HTTP PUT /account/register/
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    username: "the username provided by the user"
    identifier: "the users hwid"
}
RESPONSE: {
    status: 200/400
}
```

### Login with HWID
```go
HTTP POST /account/login/
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    identifier: "the users hwid"
}
RESPONSE: {
    status: 200/400,
    response: "the user's username"
}
```

## Discord Messages
### Send Embed
```go
HTTP POST /message/{token}/
URL_PARAMETERS: {
    token: SHA256 Encrypted Channel ID
}
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    discord_api_message_body
}
```

## Tokens
### Generate New Token
```
HTTP PUT /token
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    channel: "regular channel id as int"
}
RESPONSE: {
    status: 200/400,
    token: "generated token"
}
```

### Delete Token
```
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    token: "the token to delete"
}
```

### Get Token Data
```
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
RESPONSE: {
    status: 200/400,
    token: "token",
    channel: "channel id",
    created_by: "who created the token",
    created_at: "when the token was created",
    expires_in: "when the token expires"
}
```
