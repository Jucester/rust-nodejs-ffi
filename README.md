# NodeJS and Rust working together.

This project was created to learn how nodejs can execute Rust code through ffi and n-api.

## Rust dependencies:

* csv
* serde

## Node dependencies:

* ffi-napi


This little program uses serde and csv to parse a csv file and then convert structs to json, and return it to NodeJS.

After parsing the CSV, the program asks you to enter a Star Wars planet and then returns information about that planet.

It's pretty simple, but the idea is to see how we can use this in more complex projects.

PD: I used a CSV with Star Wars Planets, just to add more fun to the process :)
