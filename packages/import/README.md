# easy-ast.import

Fast APIs for js/ts module graph.

## Benchmark

```shell
System:
  OS: macOS 13.6
  CPU: (12) arm64 Apple M2 Pro
  Memory: 68.25 MB / 16.00 GB
  Shell: 3.2.57 - /bin/sh
Binaries:
  Node: 18.17.1 
  Yarn: 1.22.19 
  npm: 9.6.7 
  pnpm: 8.8.0 
```

|repo|file type|parsed files|total modules|import links|import depth|average time(ns)|
|---|----|-----|----|----|----|---|
|[axios@1.6.2](https://github.com/axios/axios/tree/v1.6.2)|`.js`|49|59|132|3|**12**,249,262.50|
|[rxjs@8.0.0-alpha.12](https://github.com/ReactiveX/rxjs/tree/8.0.0-alpha.12)|`.ts`|205|205|880|3|**365**,465,279.17|
|[nextui@2.0.0](https://github.com/nextui-org/nextui)|`.ts` `.tsx`|165|237|988|3|**57**,522,687.48|
|[antd@5.11.2](https://github.com/ant-design/ant-design)|`.ts` `.tsx`|557|718|3419|3|**2883**,391,816.69|

## Install

```shell
$ npm i easy-ast.import
```

## API

###

