# Key-value database

Simple study case 

## Usage

Start the server:
```
cargo run
```

## Endpoints

Store a value:
```
http://127.0.0.1:2904/query?key=mykey&value=myvalue
```

Retrieve a value:
```
http://127.0.0.1:2904/value?key=mykey
```

Check server status:
```
http://127.0.0.1:2904/status
```



