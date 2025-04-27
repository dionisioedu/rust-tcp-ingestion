
# Rust TCP Ingestion Service

Um microserviço simples em **Rust** para receber mensagens via **TCP socket**, parsear como **JSON**, e armazenar os dados no **MongoDB**.

O projeto roda via **Docker Compose** para facilitar o ambiente de desenvolvimento.

---

## 📦 Tecnologias usadas

- Rust + Tokio (async)
- MongoDB
- Docker e Docker Compose
- Python (para testes de carga)

---

## 🚀 Como rodar o projeto

### 1. Clonar o repositório

```bash
git clone https://github.com/seu-usuario/rust-tcp-ingestion.git
cd rust-tcp-ingestion
```

---

### 2. Subir os containers com Docker Compose

```bash
docker-compose up --build
```

Esse comando irá:
- Buildar o serviço Rust.
- Subir o serviço `ingestion_service` na porta `7878`.
- Subir um banco de dados MongoDB na porta `27017`.

Após o build, você verá logs como:

```
ingestion_service | Listening on 0.0.0.0:7878
mongo              | waiting for connections on port 27017
```

✅ Agora o serviço já está pronto para receber mensagens TCP.

---

### 3. Testar manualmente o envio de mensagens

No seu terminal, envie uma mensagem manual para o serviço:

```bash
echo '{"source":"sensor01","value":42.0,"timestamp":"2025-04-27T20:00:00Z"}' | nc localhost 7878
```

Isso enviará um JSON para o serviço, que será armazenado no MongoDB.

---

### 4. Teste automatizado: enviar centenas de mensagens

Existe um script Python para testes de carga: `spam_logs.py`.

#### Requisitos

- Python 3 instalado na máquina.

#### Rodar o script

```bash
python3 spam_logs.py
```

Esse script enviará **500 mensagens TCP** geradas dinamicamente para o serviço.

Cada mensagem enviada segue a estrutura:

```json
{
  "source": "sensor-<ID>",
  "value": <valor aleatório entre 10 e 100>,
  "timestamp": "<data/hora atual em UTC>"
}
```

---

### 5. Validar os dados no MongoDB

Acessar o MongoDB via terminal:

```bash
docker-compose exec mongo mongosh
```

Depois no shell do MongoDB:

```javascript
use ingestion
db.logs.countDocuments()
```

Isso mostrará quantos documentos foram inseridos.

Para visualizar os documentos:

```javascript
db.logs.find().limit(5).pretty()
```

---

## 🛠️ Estrutura do projeto

```
rust_tcp_ingestion/
├── docker-compose.yml
├── Dockerfile
├── Cargo.toml
├── src/
│   └── main.rs
└── spam_logs.py
```

---

## 🧠 Conceito principal

Este projeto demonstra:
- Como construir um microserviço TCP de alta performance usando Rust.
- Como integrar MongoDB para persistência de dados.
- Como usar Docker Compose para orquestrar múltiplos serviços.
- Como testar o fluxo de dados de ponta a ponta.

---

## 🧹 Comandos úteis

| Ação | Comando |
|:---|:---|
| Subir containers | `docker-compose up --build` |
| Derrubar containers e volumes | `docker-compose down -v` |
| Ver logs em tempo real | `docker-compose logs -f` |
| Acessar MongoDB via CLI | `docker-compose exec mongo mongosh` |

---

## 📣 Sobre

Este projeto foi desenvolvido para fins educacionais, explorando padrões modernos de desenvolvimento de microserviços assíncronos com Rust e MongoDB.

---

## ✨ Melhorias futuras (opcional)

- Suporte a autenticação no MongoDB.
- Implementar parsing robusto de mensagens (validação de schema).
- Migrar para comunicação binária (ex: Protocol Buffers).
- Suporte a TLS no TCP server.
- Dashboard de métricas (Prometheus/Grafana).

---
