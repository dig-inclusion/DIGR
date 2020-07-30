# DIGR
An automated accessibility test runner based on rules.

DIGR scans a given website to test it's accessible compenents. It is written in Rust and aim to target other languages like Javascript in future.

## Setup
* Install [Rust toolchain](https://www.rust-lang.org/tools/install).
* Make sure you have [Cargo](https://doc.rust-lang.org/cargo/) on your system.


## Usage
The DIGR binary takes multiple command line arguments, a rules yaml file, a url and a depth number:

**-r** option for rules file\
**-u** option for url\
**-d** is an number for depth in the url\

Here is an example:
`digr -r rule1.yml -u https://diginclusion.com -d 0`


#### Tests
* Cargo test


