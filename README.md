# Reborn
Versa Anti-Cheat Reborn using Rust instead of Python


# API Documentation

## Accounts
### Register with HWID
```
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
```
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
```
HTTP POST /message/{token}/
URL_PARAMETERS: {
    token: SHA256 Encrypted Channel ID
}
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    embed: "discord embed json",
    image: "base64 encoded image buffer",
    hardware_info: "base64 encoded running programs, users hwid, etc."
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
