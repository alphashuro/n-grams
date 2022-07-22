# N-gram calculator

## _warning_:

I am already aware that there is a lot of duplication and it is not designed optimally at all.
I just don't have time to make it right.

# running with the example data

- first you must clean the example data, to create plain line-separated text files
  ```shell
  cargo run --bin json-conv ./examples/fiction.json
  cargo run --bin xml-conv ./examples/reviews.xml review_text
  ```
- then you can go ahead and run the program on each of the output files

  ```shell
  cargo run ./examples/fiction.json.txt
  cargo run ./examples/reviews.xml.txt
  ```

- _Beware that the reviews contain a lot of data and take some time to execute_
  _You might want to compile the release binary first and use that instead_

  ```shell
  cargo build --release
  ./target/release/n-gram ./examples/[file]
  ```
