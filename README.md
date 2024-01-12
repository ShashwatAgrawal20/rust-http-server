# A shitty http server build for learning purposes

I am not doing any directory sanitization nor am I doing any verification if the flag is `--directory` only

### EndPoints
- http://localhost:4221/echo/{text}
- http://localhost:4221/user-agent
- http://localhost:4221/files/{filename}
- http://localhost:4221/files/{filename}


A **GET** request to files/{filename} will give back the content of the file if that exists in the directory you mentioned while starting the server using the `--directory` flag.

A **POST** request to files/{filename} with the data in the body will fetch the contents of the file from the request body and save it to <directory>/<filename> on the given directory.


### Command to Start the server
```bash
./server.sh
```
OR
```bash
./server.sh --directory `pwd`
```
**use ^ to access all 4 endpoints**

### Example Commands to Request Data

```bash
curl -v http://127.0.0.1:4221/echo/oooooooo
```
```bash
curl -v http://127.0.0.1:4221/echo/user-agent
```

**NOTE:-** In order for these commands to work you need the `--directory` flag.
```bash
curl -v http://127.0.0.1:4221/echo/files/Cargo.toml
```
```bash
curl -d 'ooooooooo' -v http://127.0.0.1:4221/files/test.txt
```
