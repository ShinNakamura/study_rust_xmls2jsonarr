# 複数のXMLを読み込んでJSONのArrayにまとめる

Rust 勉強中の自習課題。

いろいろ雑です。
クローン、利用は自己責任でお願い致します。

## 概要

- 複数のXML(UTF8)へのファイルパスを引数で受け取って
- それぞれをそのままJSONオブジェクト化(インポートしたクレート任せ)し
- 最終的にJSONの配列にまとめて標準出力へ返す。

## 例

`tests/input*.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<result>
    <message>OK</message>
    <description>
        <![CDATA[<p>
            <div class="foo">foo</div>
        </p>]]></description>
    <comment>&lt;p>bar&lt;/p></comment>
    <images><image>image_1.jpg</image><image>image_2.jpg</image></images>
    <categories>
        <categoryInfo>
            <categoryId>100</categoryId>
            <categoryName>hoge</categoryName>
        </categoryInfo>
        <categoryInfo>
            <categoryId>101</categoryId>
            <categoryName>piyo</categoryName>
        </categoryInfo>
    </categories>
</result>
```

`tests/input2.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<result>
    <message>OK</message>
    <description>
        <![CDATA[<p>
            <div class="foo">foohah</div>
        </p>]]></description>
    <comment>&lt;p>barkee&lt;/p></comment>
    <images><image>image_1.jpg</image><image>image_2.jpg</image></images>
    <categories>
        <categoryInfo>
            <categoryId>200</categoryId>
            <categoryName>hogehuga</categoryName>
        </categoryInfo>
        <categoryInfo>
            <categoryId>201</categoryId>
            <categoryName>piyopayo</categoryName>
        </categoryInfo>
    </categories>
</result>
```

コマンドラインの引数で0個以上のXMLファイルパスを指定。

```sh
cargo run -- tests/input* | jq '.[1].result.categories.categoryInfo[0].categoryId'
# "200"
```

なお、読み込むものがなくても
コマンドラインでの実行上は

- 正常終了
- 空の配列`[]`が返る

とする。

## 参考

[xmltojson](https://dev.to/rtyler/converting-xml-to-json-in-rust-1mnp)
[serde_json](https://github.com/serde-rs/json)

