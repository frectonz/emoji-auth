# Emoji Auth

An proof of concept for using emojis as a token for authentication/authorization.

```text
GET http://localhost:3000/greeting
Authorization: Emoji encodeURI('🐱🐶🐧🐤')

200 OK
Hello, 🐱🐶🐧🐤
```
