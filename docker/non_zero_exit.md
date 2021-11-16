# Docker non-zero-exit(137)

## 01 도커 빌드 중 에러 발생

로컬 환경에서 docker build 중에 아래와 같은 에러가 발생하며 build가 실패했다.

```
returned a non-zero code: 137
```

## 02 Exited (137) 종료 코드
- 128 + n은 리눅스 시그널 n 에 의해 종료되는 경우이다.
- 128 + 9로 Linux Signal num 9인 SIGKILL이 발생한 경우 (kill -9)
- docker kill, docker rm -f 로 인한 종료
- **OOM으로 강제 종료되는 경우도 SIGKILL 발생**

## 03 Docker Desktop for Mac

linux에서는 host 자원을 직접 사용할 수 있으므로 메모리에 대한 추가적인 제약이 없으나 MacOS는 LinuxKit VM 위에서 동작하므로 virtual memory의 상한선 제약을 두고 있다.

(위의 이유로 network host mode 설정이 MacOS에서는 동작하지 않고 host를 호출해야할 때는 `host.docker.internal` 주소를 사용해야한다.)

Docker Desktop for Mac 가이드를 확인해보면 아래와 같다.

> Memory: By default, Docker Desktop is set to use 2 GB runtime memory, allocated from the total available memory on your Mac. To increase the RAM, set this to a higher number. To decrease it, lower the number.
https://docs.docker.com/desktop/mac/

## 04 결론

결론적으로는 Preference > Resource 에서 가상 메모리 자원을 2G에서 4G로 증가시키면 정상적으로 빌드가 완료되었다.


아래의 조건이 만족할 때는 메모리 설정을 수정하자.
- MacOS 로컬 빌드
- 빌드 시 고용량의 패키지 설치
- exit code: 137

