# SmashDB
A distributed vector databse server supporting restful api.

## Running command
```
cargo run -- --log-level=error
```

## Usage
```
curl -s http://127.0.0.1:8080/hello/name\=world
```


## Add Facebook faiss support
```
git clone --branch v1.7.2 git@github.com:facebookresearch/faiss.git
cmake -B build -DFAISS_ENABLE_C_API=ON -DBUILD_SHARED_LIBS=ON -DCMAKE_BUILD_TYPE=Release
cmake --build build
sudo cmake --install .
sudo cp c_api/libfaiss_c.so /usr/local/lib
export LIBRARY_PATH="/usr/local/lib"

```
