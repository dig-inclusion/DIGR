# DIGR
An automated accessibility test runner based on rules.

DIGR scans a given website to test it's accessible compenents, it can be run as a standalone program, embedded as part of other program(s) or as a service like (Mongodb or Mysql).\
It is written in Rust, with an aim to provide a runtime for interpreted languages like Javascript and Typescript in future.

## Setup
* Install [Rust toolchain](https://www.rust-lang.org/tools/install).
* Make sure you have [Cargo](https://doc.rust-lang.org/cargo/) on your system.


## Usage
The DIGR binary takes multiple command line arguments, a rules yaml file, a url and a depth number:

**-r** option for rules file (__this must be yaml file__)\
**-u** option for url (__this must be a valid url__)\
**-d** is an number for depth in the url (__this must be provided if you want to test url resources__)

Here is an example:\
`digr -r rule1.yml -u https://diginclusion.com -d 0`


#### Tests
* Cargo test


