
#### ae-desugar memo

- 기존의 ae code 를 바탕으로 AST에 Neg(&neg) 추가
- interp, desugar 함수에 주의를 기울여서 보면 됨
- 특히 AST가 어떤식으로 출력되는지가 중요 : desugar 전후의 변화
- *AST는 Display trait에 구현된대로 출력됨
