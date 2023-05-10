# Reborn ![Stars](https://img.shields.io/github/stars/realTristan/Reborn?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/Reborn?label=Watchers)
![banner](https://github.com/realTristan/Reborn/assets/75189508/0be2cf20-d63d-425a-ac96-0b1907896b4d)

# About
Reborn is a secure gaming anti-cheat written in Rust and is the superior to Versa, my other anti-cheat developed in Python. With Reborn, you can easily detect cheaters, hwid/ip-spoofers, macro-users, and more.
Because Reborn is implemented entirely in Rust, you can expect fast, and reliable program connectivity.

# Program Preview
The program has two primary screens. The first is the registration screen. When an user registers with their username, they can no longer change it. Their username becomes bound to their hwid. This makes it much easier
to detect hwid spoofers who are most likely cheating. The second screen is for token input. As a tournament host, you can generate a token in your discord server, with the `/token create` discord command. This
makes it so that all data and screenshots get sent to that channel.
<div float="left">
<img src="https://user-images.githubusercontent.com/75189508/210023258-362d4ca1-9f21-4d33-b9a5-579b0796bc58.PNG" width="400" height="375">
<img src="https://user-images.githubusercontent.com/75189508/210025481-cddd53e1-99ae-4f29-a723-ac355593c81d.PNG" width="400" height="375">
</div>

# Discord Bot
The discord bot is used to create new vac tokens, delete unused, or expired tokens, and to get a tokens information. Each of these functions can be called by the tournament host
in their corresponding discord server. After the launch of Reborn, you can invite the Reborn discord bot into your server.
## Create a Token
```go
/token create
```

## Delete a Token
```go
/token delete {token}
```

## Get Token Info
```go
/token info {token}
```

# API Documentation
For clarity regarding how the program, discord bot, and server backend work, I wrote a quick api documentation to explain the functionalities behind the scenes.
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
```go
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
```go
HEADERS: {
    authorization: "user hardware id",
    access_token: SHA256("{authorization}:{time_in_seconds}:{secret_key}")
}
BODY: {
    token: "the token to delete"
}
```

### Get Token Data
```go
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

# License
MIT License

Copyright (c) 2022 Tristan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
