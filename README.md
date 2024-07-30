<img align="right" width="25%" src="docs/images/logo.svg">

# tobox

![language](https://img.shields.io/badge/language-rust-red?logo=rust&logoColor=red)
![GitHub Created At](https://img.shields.io/github/created-at/jkearnsl/tobox)
[![GitHub License](https://img.shields.io/github/license/jkearnsl/tobox)](https://github.com/JKearnsl/tobox?tab=AGPL-3.0-1-ov-file#readme)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jkearnsl/tobox/build.yml)](https://github.com/jkearnsl/tobox/actions)
[![Contributors Welcome](https://img.shields.io/badge/contributors-welcome!-blue)](https://github.com/JKearnsl/tobox)

`tobox` is a secure, high-performance, fault-tolerant decentralized object file storage written entirely in Rust.

The API is similar to [amazon s3](https://docs.aws.amazon.com/AmazonS3/latest/API/Welcome.html) but aims to be simpler.

## Todo

This is a young repository. Work plan:
- [ ] Node basic storage
- [ ] Node distributed system
- [ ] Node protection against interruption of some servers
- [ ] Node checking files based on the first byte pattern
- [ ] Panel


## Synchronizing objects

In order to avoid conflicts and reduce the overhead costs associated with ensuring the uniqueness of file names 
in a decentralized virtual directory, it has been decided to abandon this requirement 
and instead use the file's name as a unique identifier.

The new approach involves generating a unique object key in UUIDv4 format and using it to access files in the future. 
This method has several advantages beyond avoiding synchronization issues:

* In some cases, such as storing large volumes of user photos or public documents, it eliminates the need to ensure file name uniqueness. This can save time and resources that would otherwise be spent on unnecessary checks.
* Clients often assign unique identifiers to files themselves (e.g., UUID) and maintain a separate table in a relational database that links files to user IDs. With the new approach, all files start with a unique UUIDv4 and optional filename/pathname, which users can choose to use or not.

Tobox approach also has a significant drawback when using cloud file storage as a local one. 
Users can get confused if they see two files with the same name in the same directory. 
This problem can be partially solved by checking if the filename exists in the specified pathname. 
However, **tobox does not guarantee** the uniqueness of the filename and path name pair, **but rather allows it**.
