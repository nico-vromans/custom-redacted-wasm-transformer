# custom-redacted-wasm-transformer

This is a custom RedactedTransformer for [RepliByte], aiming to solve [this bug].
This bug has already been fixed in [this PR], but no new release has been made since.

You could compile the [main] branch (which has the fix in it) yourself, that could also work. This has been tested
in [this fork] (the Windows build failed, but I didn't look it).

In order to compile and use this transformer, you can follow along with [these instructions], but this is a slightly
updated method (primarily the target):

- `rustup target add wasm32-wasip1`
- `cargo build --release --target wasm32-wasip1`
- update your <CONFIG>.y(a)ml to reflect th path:
    ```yaml
    source:
      # ...
      transformers:
        - database: <DATABASE>
          table: <TABLE>
          columns:
            - name: <COLUMN_NAME>
              transformer_name: custom-wasm
              transformer_options:
                path: "path/to/custom-redacted-wasm-transformer/target/wasm32-wasip1/release/custom-redacted-wasm-transformer.wasm"
    ```

[//]: # (Links)

[RepliByte]: https://www.replibyte.com/

[this bug]: https://github.com/Qovery/Replibyte/issues/275

[this PR]: https://github.com/Qovery/Replibyte/pull/279

[main]: https://github.com/Qovery/Replibyte/tree/main

[these instructions]: https://www.replibyte.com/docs/advanced-guides/web-assembly-transformer/

[this fork]: https://github.com/nico-vromans/Replibyte/releases/tag/v0.10.1