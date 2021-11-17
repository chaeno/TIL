# API GATEWAY 403 Error

## 에러 상황
Zappa를 사용하여 배포한 flask 기반 lambda가 default vpc 내에서 호출 시 403 Forbidden이 발생했다. 이상하게도 local 환경과 default vpc가 아닌 vpc에서는 정상 호출이 가능했다.

## 원인
Zappa는 lambda를 외부로 노출하기 위해 api gateway에 연동하고 기본적으로 엔드포인트 유형을 edge로 설정한다. edge API는 default vpn에서는 호출되지 않으므로 private DNS를 사용할 수 있도록 private API로 변경이 필요하다.

## 해결 방법
1. API Gateway에서 가져오기를 통해 기존의 edge API를 private API로 변경하여 생성한다.
2. 해당 API의 리소스 정책에서 excute-api:Invoke를 허용해준다.
3. 리소스에서 API를 배포한다.
4. 생성된 url로 변경하여 호출한다.
Zappa를 사용하여 배포한 flask 기반 lambda는 api gateway 연동 시 엔드포인트 타입을 Edge로 사용한다.