# API GATEWAY 403 Error

## 01 에러 상황
Zappa를 사용하여 배포한 flask 기반 lambda가 default vpc 내에서 호출 시 403 Forbidden이 발생했다. 이상하게도 local 환경과 default vpc가 아닌 vpc에서는 정상 호출이 가능했다.

## 02 원인

HTTP 403 응답 코드는 클라이언트에서 유효한 URL에 엑세스하는 것이 금지되었음을 뜻한다. 사유는 다양할 수 있으므로 응답 헤더와 오류 메세지로 원인을 좁혀야한다.

https://aws.amazon.com/ko/premiumsupport/knowledge-center/api-gateway-troubleshoot-403-forbidden/

위 문서에 403 에러 관련 정보가 있고 이번 에러 케이스는 아래와 같았다.
- 응답 헤더: "x-amzn-ErrorType" = "ForbiddenException"
- 응답 메세지: "Forbidden"

해당 케이스에는 총 4가지의 상황이 존재한다.
1. API 키가 유효하지 않음
2. AWS WAF가 필터링됨
3. 퍼블릭 DNS 이름을 잘못 사용하여 프라이빗 API 호출
4. 기본 execute-api 엔드포인트를 사용하여 사용자 지정 도메인 이름을 가진 REST API를 호출합니다.

1번은 API 키 사용이 활성화되어 있지 않으므로 해당하지 않는다.
2번은 AWS WAF가 설정된 프로젝트가 아니고 에러 발생해도 WAF에 잡히지 않으므로 해당하지 않는다.
4번은 기본 execute-api 엔드포인트를 비활성화한적이 없으므로 해당하지 않는다.
3번이 적합한 원인으로 보인다.

3번과 관련된 내용은 아래와 같은데 우리는 private 환경에서 public DNS 호스트네임을 호출하는 것이 문제였다.
https://docs.aws.amazon.com/ko_kr/apigateway/latest/developerguide/apigateway-private-api-test-invoke-url.html#apigateway-private-api-public-dns

Zappa는 lambda를 외부로 노출하기 위해 api gateway에 연동하고 기본적으로 엔드포인트 유형을 edge로 설정한다. edge API는 default vpc에서는 호출되지 않았고, private DNS를 사용할 수 있도록 private API로 변경이 필요했다.

## 03 해결 방법
1. API Gateway에서 가져오기를 통해 기존의 edge API를 private API로 변경하여 생성한다.
2. 해당 API의 리소스 정책에서 excute-api:Invoke를 허용해준다.
3. 리소스에서 API를 배포한다.
4. 생성된 url로 변경하여 호출한다.

## 04 질문

Q1. edge 엔드포인트 유형의 API는 왜 default vpc에서 호출되지 않는걸까?
