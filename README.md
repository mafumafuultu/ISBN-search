# ISBN-search
Get book information from the CLI

APIs in use : [国立国会図書館サーチ APIのご利用について](https://iss.ndl.go.jp/information/api/)


## USE


```shell
isbn 978-4-00-310101-8
# OR
isbn 9784003101018
```

output
```
"identifier" : https://iss.ndl.go.jp/books/R100000002-I000002041889-00 
"title" : 吾輩は猫である
"creator" : 夏目漱石 作
"creator" : 夏目, 漱石, 1867-1916
"publisher" : 岩波書店
"price" : 410円 (税込)
"identifier" : 4003101014
"identifier" : 90035836

...
```
