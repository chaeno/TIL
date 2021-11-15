# Persional Access Token
```
remote: Support for password authentication was removed on August 13, 2021. Please use a personal access token instead.
remote: Please see https://github.blog/2020-12-15-token-authentication-requirements-for-git-operations/ for more information.
fatal: Authentication failed for 'https://github.com/chaeno/private.git/'
```
## 01 배경
GitHub은 기존에 사용하던 username/password 기반의 인증 방식을 더 이상 사용하지 않는다.

위 메세지의 blog(https://github.blog/2020-12-15-token-authentication-requirements-for-git-operations/) Background 내용은 여러 보안 강화를 노력해 왔음에도, 이를 활성화하지 않는 사용자는 오직 username/password만으로 Git auth와 API operation을 사용할 수 있는 문제점이 있었다.

이를 해결하기 위해 GitHub은 단계적으로 Token-based의 인증방식을 사용하도록 Timeline을 정해 업데이트를 진행해왔고 2021-08-13일부로 Persional access token 또는 OAuth token을 사용해야만 인증된 Git operation을 사용할 수 있다.

## 02 사용법
1. GitHub.com 로그인
2. 우상단의 Profile 영역 클릭
3. Settings 클릭
4. 좌측 Developer Setting 클릭
5. 좌측 Persional access token 클릭
6. Generate new token
7. 유효기간과 필요한 권한 체크 
8. Generate token
9. username/token으로 인증




