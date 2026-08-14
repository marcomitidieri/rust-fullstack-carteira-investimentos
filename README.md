# Carteira de Investimentos

## O que o projeto faz

Aplicação web full-stack desenvolvida em Rust para gerenciamento de uma carteira de investimentos.

O projeto possui:

- autenticação de usuários com JWT e cookies;
- API para gerenciamento de ativos;
- persistência dos dados em PostgreSQL;
- interface web simples para login;
- testes automatizados utilizando SQLx.

Durante meus estudos, implementei novas operações na API de ativos, incluindo consulta e exclusão por ID, além de melhorias no tratamento de erros e um teste automatizado para a exclusão.

## Como executar a aplicação

### 1. Subir o banco de dados

O projeto utiliza PostgreSQL. Caso esteja utilizando o `compose.yml` fornecido pelo projeto:

```bash
docker compose up -d
```

Obs.: Caso não tenha o Docker, instale e levante o serviço do PostgreSQL no computador local.

### 2. Configurar o banco

Configure a variável `DATABASE_URL` no arquivo `.env`:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
```

### 3. Executar as migrations

```bash
sqlx migrate run
```

### 4. Executar a aplicação

```bash
cargo run
```

A aplicação será disponibilizada em `localhost:3000`.

## Quais tecnologias foram usadas

- **Rust** — linguagem de programação;
- **Axum** — framework web;
- **Tokio** — runtime assíncrono;
- **SQLx** — acesso ao PostgreSQL;
- **PostgreSQL** — banco de dados;
- **Askama** — geração de HTML;
- **JWT** — autenticação;
- **Docker Compose** — ambiente do banco de dados;
- **Serde** — serialização e desserialização de dados;
- **tracing** — observabilidade;
- **insta / SQLx** — testes.

## Quais melhorias você implementou

Durante os estudos, implementei novas funcionalidades na API de ativos e melhorei o tratamento de erros.

### 1. Pesquisa de ativo por ID

**Rota:** `GET /api/assets/{id}`

Foi adicionada uma nova operação para consultar um ativo específico pelo seu ID.

Alterações realizadas:

- criação do método `get_asset_by_id` no `Repository`;
- criação do handler `get_asset_by_id`;
- criação da rota `GET /assets/{id}`;
- retorno do ativo em formato JSON quando encontrado;
- retorno de erro `404` quando o ativo não existe.

### 2. Exclusão de ativo por ID

**Rota:** `DELETE /api/assets/{id}`

Foi adicionada a possibilidade de excluir um ativo pelo seu ID.

Alterações realizadas:

- criação do método `delete_asset` no `Repository`;
- utilização da cláusula SQL `RETURNING` para retornar os dados do ativo excluído;
- criação do handler `delete_asset`;
- criação da rota `DELETE /assets/{id}`;
- exigência de autorização de administrador para realizar a exclusão;
- retorno de erro `404` quando o ativo não existe.

### 3. Melhoria no tratamento de erro

O erro `AssetDoesNotExist` foi alterado para receber o ID do ativo não encontrado.

Antes:

```rust
AssetDoesNotExist,
```

Depois:

```rust
#[error("Ativo com id {0} não encontrado")]
AssetDoesNotExist(i64),
```

Com isso, a aplicação passa a fornecer informações mais específicas sobre o recurso que não foi encontrado.

Por exemplo:

```text
Ativo com id 999 não encontrado
```

### 4. Teste automatizado da exclusão

Foi criado o teste `test_delete_asset` em `api.rs` utilizando `sqlx::test` e a fixture `bitcoin_asset`.

O teste executa o handler de exclusão e verifica se os dados do ativo retornado correspondem ao registro esperado.


### Arquivos modificados

| Arquivo | Principais alterações |
|---|---|
| `error.rs` | Alteração de `AssetDoesNotExist` para receber o ID do ativo |
| `repository.rs` | Inclusão das operações de consulta e exclusão por ID |
| `api.rs` | Inclusão das rotas, handlers e teste de exclusão |

## Como testar sua versão

### Testes automatizados

Para executar todos os testes:

```bash
cargo test
```

Para executar especificamente o teste de exclusão:

```bash
cargo test test_delete_asset
```

### Teste da pesquisa por ID

Com a aplicação em execução:

```bash
curl http://localhost:3000/api/assets/1
```

Se o ativo existir, a API retorna os dados do ativo em JSON.
Se o ID não existir, a API retorna 404 (Not Found).

Para consultar um ID inexistente:

```bash
curl http://localhost:3000/api/assets/999
```

Se o ativo existir, a API exclui o registro e retorna os dados do ativo excluído em JSON.
Se o ID não existir, a API retorna 404 (Not Found).

### Teste da exclusão por ID

Para excluir um ativo pelo seu ID:

```bash
curl -X DELETE http://localhost:3000/api/assets/1 -H "Authorization: im-the-admin"
```

A operação exige autorização de administrador.

## O que eu aprendi durante o desafio

Este projeto foi uma oportunidade de aplicar Rust em uma aplicação real, trabalhando com API, banco de dados, autenticação e testes.

Durante a implementação, aprofundei meus conhecimentos sobre:

- criação de rotas e handlers com Axum;
- utilização de `Path`, `Result` e `Option`;
- consultas parametrizadas com SQLx;
- operações de banco de dados utilizando PostgreSQL;
- tratamento de erros através de enums;
- autenticação e proteção de endpoints;
- criação de testes automatizados com `sqlx::test`;
- utilização de fixtures para fornecer dados aos testes;
- execução de testes com `cargo test`.

A principal experiência deste estudo foi perceber como uma alteração aparentemente simples, como adicionar uma rota de exclusão, envolve diferentes partes da aplicação: rota HTTP, handler, autorização, repositório, banco de dados, tratamento de erros e testes.