# Guia de Contribuição: Steam Companion 🐢

Obrigado pelo seu interesse em contribuir para o **Steam Companion**! Este documento orienta sobre o fluxo de desenvolvimento, os padrões de código e as boas práticas de commit.

---

## 🌟 Princípios de Desenvolvimento

1. **Rust Idiomático e Seguro:** Priorize segurança de tipos, controle estrito de erros com `anyhow`/`thiserror` e zero alocações desnecessárias.
2. **Alta Performance:** O launcher deve manter uso residual de memória (~40MB de RAM) e extensões de hardware opt-in para não competir com jogos e emuladores.
3. **Cobertura de Código Obrigatória (≥ 70%):** Todo novo recurso, parser ou módulo deve ser acompanhado de testes unitários ou de integração, mantendo a cobertura do workspace em ou acima de 70%.

---

## 🛠 Configuração do Ambiente

1. Certifique-se de ter o compilador Rust instalado:

   ```bash
   rustup update stable
   rustup component add llvm-tools-preview
   ```

2. Instale as ferramentas auxiliares:

   ```bash
   cargo install cargo-llvm-cov --locked
   ```

3. Clone o repositório recursivamente:

   ```bash
   git clone --recurse-submodules https://github.com/marcoswferreira/kame-house-launcher.git
   cd kame-house-launcher
   ```

---

## 🧪 Validação e Testes Locais

Antes de abrir um Pull Request ou criar commits:

1. **Checar Formatação e Lints:**

   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   ```

2. **Executar a Suíte de Testes:**

   ```bash
   cargo test --workspace
   ```

3. **Verificar a Cobertura de Código:**

   ```bash
   cargo llvm-cov --workspace --summary-only
   ```

   A cobertura total de linhas não pode ser inferior a 70%.

---

## 📝 Padrão de Commits (Conventional Commits)

Todos os commits devem seguir a especificação [Conventional Commits v1.0.0](https://www.conventionalcommits.org/):

```text
<tipo>(<escopo opcional>): <descrição no imperativo e em minúsculas>

[corpo opcional explicando o porquê da alteração]

[rodapé opcional para issues resolvidas ou breaking changes]
```

### Tipos Permitidos

- `feat`: Novo recurso para o usuário ou biblioteca (ex.: novo parser de console, novo preset de TV).
- `fix`: Correção de bug.
- `docs`: Alterações exclusivamente na documentação ou ADRs.
- `style`: Formatação, pontuação ausente, sem alteração de código produtivo.
- `refactor`: Refatoração de código que não adiciona recurso nem corrige bug.
- `perf`: Mudança de código que melhora o desempenho.
- `test`: Adição de testes faltantes ou correção de testes existentes.
- `chore`: Atualizações de build, dependências ou ferramentas de CI/CD.

### Exemplos de Boas Mensagens de Commit

- `feat(core): add rom header detection for sega saturn disc images`
- `fix(steam): correct vdf binary endianness when writing 64-bit integers`
- `docs: update dualsense pairing instructions in readme`
- `test(console): add unit tests for display resolution presets`

---

## 🔀 Fluxo de Branches e Pull Requests

1. Crie uma branch descritiva a partir de `main`:

   ```bash
   git checkout -b feat/gamepad-virtual-keyboard
   ```

2. Realize alterações atômicas com commits bem descritos.
3. Certifique-se de que `cargo test --workspace` e `cargo llvm-cov` passem sem erros.
4. Abra o Pull Request descrevendo:
   - Qual problema ou necessidade a alteração resolve.
   - Como foi testado.
   - Impacto na performance ou nos subsistemas relacionados.
