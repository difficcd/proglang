### 기본 명령어 정리  

[ cmd/powershell ]

 **cargo init**
> 해당 dir에서 빌드할 수 있게 세팅해줌      
    cargo init 이후 ls를 해보면 Cargo.toml(설정파일), src(dir) 생성됨    
    여기서 vim Cargo.toml 으로 설정파일 읽기/편집이 가능하며,    
    src dir 에는 기본적으로 main.rs가 생성됨을 확인할 수 있다.

 project dir: **cargo build**  
> 빌드하면 해당 dir에 추가적으로 Cargo.lock, target 이 생성된다    
   여기서 cd target 에서 ls를 해보면 bedug dir 존재 (이하 생략)

project dir: **cargo run**  
> 실행 명령어로, rs 파일을 실행시켜 결과를 내놓게 함  
  이때 cargo 가 내는 출력을 보고싶지 않으면 **cargo -q run** 으로 실행가능  

 </ br>

- ae/ 에서 cargo build 한 이후 toml 파일을 보면 lalrpop 이라는 lib 을 쓰는것을 알 수 있음    
- LALR 은 파싱을 위한 library. build.rs 또한 lalrpop 을 쓰기 위한 파일  
- ae.lalrpop : lalr lib 에서 입력받아 사용. 파서를 만드는 코드 (grammar 받아서 파서 만들어줌)  
- ast.rs는 AST 를 Rust 구조체로 정의한 파일.
- ae.lalrpop은 ast에 있는 code 를 가져다 씀 (결국 AST 를 만들어야 하기 떄문)
- main.rs는 AST 를 만들고 실행시키는 코드  

  
***
   
   
* cd \ : root dir,  cd .. : parent dir )
*  mkdir project,  cd project/  del project 

* vim 기초 명령어  
  (1)  모드 :  입력 모드 i  명령 모드 Esc  저장/종료 :  
  (2)  저장/종료 :w 저장, :q 종료, :wq/:x 저장 후 종료   
                 :q! 강제 종료  ZZ(=:wq, i모드일때 가능)   
