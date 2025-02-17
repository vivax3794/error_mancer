# 0.4.3
* **feature:** Now correctly works on async functions.

# 0.4.2
* **feature:** You can now set an explicit enum name by providing a ident instead of `_` as the error type in the signature.
* **Fix**: Some part of the generated code didnt have `::...` for a stdlib reference, which in theory could leave it open to name shadowing and hence breaking.
* **Cleanup**: Make docs nicer

# 0.4.1
* Added support for annotating error types with `#[derive]` attribute to derive extra traits.

# 0.4.0
* Fixed variant name construction: Did not actually consider each path segment as a proper word.
* Cleaned up variant names: Now strips "Error" suffix if present.

| path | 0.3.1 | 0.4.0 |
| --- | --- | --- |
| `std::io::Error` | `StdioError` | `StdIo` |
| `io::Error` | `IoError` | `Io` |
