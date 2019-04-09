# CannyLSのissue28を再現する

issueそのものについては [issue28](https://github.com/frugalos/cannyls/issues/28) を参照してください。

## 手順
```bash
$ cargo build --all

$ rm -f test.lusf

$ ./target/debug/get_unexpected_data --phase=1
put `hoge` to lump_id1(= LumpId("00000000000000000000000000000457")).
delete lump_id1(= LumpId("00000000000000000000000000000457")).
put `foo` to lump_id2(= LumpId("0000000000000000000000000153158e")).

$ ./target/debug/get_unexpected_data --phase=2
try to read a datum from lump_id1(= LumpId("00000000000000000000000000000457")).
We deleted lump_id1(= LumpId("00000000000000000000000000000457")); however, we can read a datum from lump_id1.
Furthermore, the read data `foo` is not `hoge`.
```

## メッセージの説明
1. `phase=1`では以下のことを行う:
    a. `lump_id1`に"hoge"をputする。
    b. この段階でjournalをdiskに同期する。
    c. `lump_id1`をdeleteする。
    d. `lump_id2`に"foo"をputする。
    e. `mem::forget`を呼び出して、プロセスのクラッシュを模倣する。
2. `phase=2`では以下のことを行う:
    a. `lump_id1`をgetしようとする。
    b. 削除した筈の`lump_id1`からデータが読み込める。
    c. 読み込むデータは"hoge"ではなく"foo"である。

## 原因
* `lump_id1`をdeleteした段階では、deleteした情報はdiskに永続化されていない。
* 一方で、メモリアロケータは"hoge"の位置にデータを書くことができる。
* 結果的に、`lump_id2`に対するputで"hoge"のあった位置に"foo"を書き込んでしまう。
* 再起動後には、`lump_id1`は生存しているように見えてしまい、結果として"foo"を読み出す。