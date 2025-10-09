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

   
   
   
* cd \ : root dir,  cd .. : parent dir )
*  mkdir project,  cd project/  del project 

* vim 기초 명령어  
  1) 모드 :  입력 모드 i  명령 모드 Esc  저장/종료 :  
  2) 저장/종료 :w 저장, :q 종료, :wq/:x 저장 후 종료  
               :q! 강제 종료  ZZ(=:wq, i모드일때 가능)
