# JSON Web Token Authentication with Axum

Based on [github.com/tokio-rs/axum/examples/jwt](https://github.com/tokio-rs/axum/tree/main/examples/jwt)

```bash
# terminal 1
make serve

# terminal 2
make token
# {"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjIwMDAwMDAwMDB9.ULPZ0NLBq9tfHroRgxJJeEYCy0tguZrEwix3fo-2dFc","token_type":"Bearer"}

make protected_request
{"sub":"b@b.com","company":"ACME","exp":10000000000}


BEARER=123 make protected_request
# {"error":"Invalid token"}
```

## TODO

- [ ] `register/` route
- [ ] .
- [ ] .
- [ ] .
- [ ] .
- [ ] .

