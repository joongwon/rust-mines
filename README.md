# Rust로 구현한 지뢰찾기

코파일럿과 함께 Rust로 지뢰찾기를 구현해보았습니다.

## 실행

일단 cargo가 있어야 하구요.

* wasm-pack: 다음을 실행하여 설치

``` curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh ```

빌드하는 법:

```bash
wasm-pack build --target web
```

실행하는 법:

```bash
# 예시:
npx serve

# 사실 뭘 쓰든간에 파일 호스팅이 되면 됩니다.
```

Ubuntu & Firefox로 이렇게 했더니 잘 된 것 같습니다.

따닥이 잘 안 돼서 불-편하지만 몰라 나중에 고쳐야죠.