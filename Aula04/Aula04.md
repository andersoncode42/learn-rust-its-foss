# Aula 04 - Arrays e Tuplas

- Arrays: Armazena múltiplos valores do **mesmo** tipo
- Tuplas: Armazena múltiplos valores, sejam do mesmo tipo ou mesmo de tipos diferentes

## Arrays

- Cada elemento do array **deve** ser do **mesmo** tipo
- Arrays tem tamanho fixo
- Arrays são armazenados na pilha, ou seja, os dados armazenados nela podem ser acessados ​​rapidamente

Sintaxe:

```rust
// without type annotation
let variable_name = [element1, element2, ..., elementn];

// with type annotation
let variable_name: [data_type; array_length] = [element1, element2, ..., elementn];


// Um elemento repetido n vezes
let variable_name = [element; qtd_copias];
```

## Tuplas