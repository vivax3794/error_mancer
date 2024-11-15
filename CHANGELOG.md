# 0.4.0
* Fixed variant name construction: Did not actually consider each path segment as a proper word.
* Cleaned up variant names: Now strips "Error" suffix if present.

| path | 0.3.1 | 0.4.0 |
| --- | --- | --- |
| `std::io::Error` | `StdioError` | `StdIo` |
| `io::Error` | `IoError` | `Io` |
