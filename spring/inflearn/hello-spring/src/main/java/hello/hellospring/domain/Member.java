package hello.hellospring.domain;

public class Member {

    // id 는 시스템이 정한다.
    // name 은 외부로부터 입력 받는다.
    private Long id;
    private String name;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }
}
