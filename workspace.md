# `Workspace`機能を使ってみる

一つのプロジェクトの開発が進むにつれ、Rustで書かれたプログラムを分割したいことがある。
そこで、`workspace`機能を使ってみる

## `workspace`ってそもそもなにさ

同じ`Cargo.lock`と出力ディレクトリを共有する一連のパッケージ。
例えば、以下の構造が考えられる。

```sh
.
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

これはどうなっているかというと、
まず、ルートの`Cargo.toml`に
```toml
[workspace]

members = [
    "adder",
]
```
が設定されている上で、
```sh
cargo new --bin adder
```

を実行すると、あのような状態になる。
コンパイル成果物は、まとめて、ルートの`target`ディレクトリに配置されることになる。
ワークスペースのクレートはお互いに依存しあうため、まとめて生成される。
各クレートが各々の`target`ディレクトリを持っていたら、不必要な再ビルドの必要が生じるので、
この構造になっているということです。
（そのため、一部を切り出してビルドした成果物が欲しいなら、それぞれをGit Submoduleにしてしまった方が良さそう）

さらに、ここからさらに、`workspace`にクレートを追加することができる。
これは、バイナリクレートでも、ライブラリクレートでもよい。

だから、このようにできる
```sh
.
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

