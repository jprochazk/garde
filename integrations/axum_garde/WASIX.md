Garde now fully supports running in WebAssembly using axum on the browser or from the Edge.

Just compile with this...

```
cargo wasix build --examples
```

You can then run it with this...
```
wasmer run --net target/wasm32-wasmer-wasi/debug/examples/json.wasm
```

Try some examples, like this:

```
user@laptop:/prog/garde$ echo "{ \"username\": \"bob\", \"password\": \"toosmall\" }" | curl --data-binary @- -H "Content-Type: application/json" http://127.0.0.1:8080/person
value.password: length is lower than 15
```

```
user@laptop:/prog/garde$ echo "{ \"username\": \"bob\", \"password\": \"asjdnfkjasndfkjansdf\" }" | curl --data-binary @- -H "Content-Type: application/json" http://127.0.0.1:8080/person
{"username":"bob","password":"asjdnfkjasndfkjansdf"}
```

You can also test it on Wasmer Edge like this:

```
user@laptop:/prog/garde$ echo "{ \"username\": \"bob\", \"password\": \"asjdnfkjasndfkjansdf\" }" | curl --data-binary @- -H "Content-Type: application/json" https://garde.wasmer.app/person
```