A [nushell](https://www.nushell.sh) plugin for converting charsets.

### Installation

```bash
cargo install nu_plugin_charset
register ~/.cargo/bin/nu_plugin_charset
```

### Usage

* Use `charset` to detect charset of input.
    ![screenshot](./img1.png)

* Use `charset decode` to convert input to utf string.
    ![screenshot](./img2.png)

    or convert to a given charset
    ![screenshot](./img3.png)


* Use `charset encode ENCODING_NAME` to convert utf string to a given charset
    ![screenshot](./img4.png)
