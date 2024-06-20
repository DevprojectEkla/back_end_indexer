# Rust Backend Application for File Indexing with TF-IDF

## Description

While this Rust program could be more extensively implemented to be used as a cli application I rather designed it as a basic library for indexing files using TF-IDF (Term Frequency-Inverse Document Frequency). It provides functionality to traverse directories, extract metadata, calculate TF-IDF scores, and index files for efficient searching and retrieval.  
I like to see it as a backend for my Rust client application and it could easily become one but it surely is an API providing mainly the class `Index` (a `Struct` in Rust) implementing the `.index_all()` method that a client app can use.


>   The code is freely inspired by this public [repo](https://github.com/tsoding/seroost/) from tsoding.  
>   I added functionnalities to crawl in multiple directories and parse every file in a direcotory tree, ignoring binaries.  

## Features

- **File Crawling:** Traverse directories recursively to discover files.
- **Metadata Extraction:** Extract essential metadata such as file size, modification date, and type.
- **TF-IDF Calculation:** Compute TF-IDF scores to represent the importance of terms in files.
- **Indexing:** Build an index of files based on their contents and TF-IDF scores.
- **Search Capability:** Implement search functionality using TF-IDF scores to retrieve files based on queries.

## Technology Stack

- **Language:** Rust  
- **Libraries/Frameworks:**  
  - serde: A framework for serializing and deserializing Rust data structures efficiently.  
  - serde_json: Provides functions to parse JSON data and convert it into Rust data structures and vice versa.  
  - walkdir: A simple Rust crate for iterating over directories recursively.  
  - poppler: A Rust binding for the Poppler PDF rendering library, used for PDF file handling and text extraction.  

## Installation

### Prerequisites

- Rust installed on your system ([Install Rust](https://www.rust-lang.org/tools/install)).

### Clone Repository

```bash
git clone https://github.com/DevprojectEkla/back_end_indexer.git
cd back_end_indexer
```
## Dependencies

Add the following dependencies to your `Cargo.toml` file:

```bash
toml
[dependencies]
rand = "0.8.5"
poppler = "0.3.2"
serde_json = "1.0.108"
serde =  { version = "1.0.190", features = ["derive"] }
walkdir = "2.4.0"
xml-rs = "0.8.19"
```

### Build and Run

>  there you can simply test basic functions and functionnalities
>  of the library like indexing a directory, calculating the tf-idf ratio ...
>  edit main.rs file to do so 

```bash
cargo build --release
cargo run
```

## Usage

### How It Works

1. **Indexing Files with TF-IDF:**
   - The application starts by crawling specified directories.
   - For each file encountered, it extracts metadata and calculates TF-IDF scores for terms.
   - Files are indexed based on their TF-IDF scores.

###  With the associated RustIndexer Client app'

2. **Searching Files Using TF-IDF:**
   - Users can search for files based on keywords or terms with high TF-IDF scores.
   - Results are returned based on relevance to the search query using TF-IDF scores.

## Future Improvements

- Implement additional text preprocessing techniques for better TF-IDF calculations.
- Enhance search functionality with advanced querying options.
- Optimize indexing process and search algorithms for large file sets.

## Contribution

Contributions are welcome! If you find any issues or have suggestions for improvements, please submit a pull request or open an issue on [GitHub](https://github.com/DevprojectEkla/back_end_indexer).

## License

This project is licensed under the [GPL-3.0 license](https://github.com/DevprojectEkla/back_end_indexer/blob/main/LICENSE).
