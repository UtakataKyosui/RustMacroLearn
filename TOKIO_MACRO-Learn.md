# `tokio`のマクロとか読んでみる

## よく使われている`#[tokio::main]`ってどのようなコードなのか

`tokio_macros/lib.rs`に、以下のコードが書かれている

```rust
#[proc_macro_attribute]
pub fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    entry::main(args.into(), item.into(), true).into()
}
```

がその中身になっている。
そして、この`entry::main`がどのようなものかというと

```rust
pub(crate) fn main(args: TokenStream, item: TokenStream, rt_multi_thread: bool) -> TokenStream {
    // If any of the steps for this macro fail, we still want to expand to an item that is as close
    // to the expected output as possible. This helps out IDEs such that completions and other
    // related features keep working.
    let input: ItemFn = match syn::parse2(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };

    let config = if input.sig.ident == "main" && !input.sig.inputs.is_empty() {
        let msg = "the main function cannot accept arguments";
        Err(syn::Error::new_spanned(&input.sig.ident, msg))
    } else {
        AttributeArgs::parse_terminated
            .parse2(args)
            .and_then(|args| build_config(&input, args, false, rt_multi_thread))
    };

    match config {
        Ok(config) => parse_knobs(input, false, config),
        Err(e) => token_stream_with_error(parse_knobs(input, false, DEFAULT_ERROR_CONFIG), e),
    }
}
```

このようになっている。



