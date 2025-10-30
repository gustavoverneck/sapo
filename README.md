### SAPO

This repository is an experimental, personal implementation of an ERP system (my own take on SAP-style functionality).
The goal is to learn and make architectural decisions by building small, focused components over time.

Quick start (development)

1. Ensure you have Rust and Cargo installed (https://www.rust-lang.org/tools/install).
2. From the project root, build and run the backend:

```bash
cd d:\codes\sapo\hiperon-backend
cargo run
```

The Actix Web server listens on 127.0.0.1:8080 by default. Open http://127.0.0.1:8080/ in your browser to verify it's running.

Notes

- This project is a work in progress. Expect many small iterations and refactors.
- Contributions or suggestions are welcome — open an issue with ideas or improvements.


## To-do List
- [ ] Estruturar um banco de dados de usuários
- [ ] Estruturar um sistema de autenticação
- [ ] Implementar um sistema de Login