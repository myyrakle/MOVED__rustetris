# rustetris

Yew.rs, Canvas로 구현된 웹 테트리스
      
프로젝트는 [별도 조직](https://github.com/tetrust)으로 이전함   
 
## 실행법

기본설정

```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

개발모드

```
trunk serve --open
```

배포 빌드

```
trunk build --release
```
