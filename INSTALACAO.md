# Guia de Instalação - Pong with Rust

Este guia fornece instruções detalhadas para configurar o ambiente e executar o jogo.

## Passo 1: Instalar Rust

### Linux e macOS

Abra o terminal e execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Siga as instruções na tela. Após a instalação, reinicie o terminal ou execute:

```bash
source $HOME/.cargo/env
```

### Windows

1. Baixe o instalador do Rust em [rustup.rs](https://rustup.rs/)
2. Execute o instalador e siga as instruções
3. Reinicie o terminal após a instalação

### Verificar Instalação

```bash
rustc --version
cargo --version
```

Você deve ver as versões instaladas do Rust e Cargo.

## Passo 2: Instalar SDL2

### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install libsdl2-dev libsdl2-mixer-dev libsdl2-ttf-dev
```

### Fedora

```bash
sudo dnf install SDL2-devel SDL2_mixer-devel SDL2_ttf-devel
```

### Arch Linux

```bash
sudo pacman -S sdl2 sdl2_mixer sdl2_ttf
```

### macOS (Homebrew)

Primeiro, instale o Homebrew se ainda não tiver:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Depois instale o SDL2:

```bash
brew install sdl2 sdl2_mixer sdl2_ttf
```

Configure a variável de ambiente (adicione ao seu `~/.zshrc` ou `~/.bash_profile`):

```bash
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

Recarregue o arquivo de configuração:

```bash
source ~/.zshrc  # ou source ~/.bash_profile
```

### Windows (MSVC)

1. Baixe as bibliotecas de desenvolvimento SDL2:
   - SDL2: https://github.com/libsdl-org/SDL/releases
   - SDL2_mixer: https://github.com/libsdl-org/SDL_mixer/releases
   - SDL2_ttf: https://github.com/libsdl-org/SDL_ttf/releases

2. Extraia os arquivos e copie as DLLs para o diretório do projeto

3. Configure as variáveis de ambiente conforme necessário

**Alternativa para Windows:** Use o recurso `bundled` do crate SDL2 (já configurado no projeto para facilitar).

## Passo 3: Clonar ou Extrair o Projeto

Se você recebeu o projeto em um arquivo ZIP:

```bash
unzip pong-rust.zip
cd pong-rust
```

Se está em um repositório Git:

```bash
git clone <url-do-repositorio>
cd pong-rust
```

## Passo 4: Compilar o Projeto

### Modo Debug (mais rápido para compilar)

```bash
cargo build
```

### Modo Release (otimizado para performance)

```bash
cargo build --release
```

## Passo 5: Executar o Jogo

### Modo Debug

```bash
cargo run
```

### Modo Release (recomendado)

```bash
cargo run --release
```

## Resolução de Problemas

### Erro: "SDL2 not found"

**Linux:** Certifique-se de que instalou os pacotes `-dev` ou `-devel` do SDL2.

**macOS:** Verifique se a variável `LIBRARY_PATH` está configurada corretamente.

**Windows:** Considere usar a feature `bundled` adicionando ao `Cargo.toml`:

```toml
[dependencies]
sdl2 = { version = "0.38.0", features = ["mixer", "ttf", "bundled"] }
```

### Erro: "Font not found"

Certifique-se de que o arquivo `assets/fonts/retro.ttf` existe no diretório do projeto.

### Erro de Compilação

Verifique se está usando uma versão recente do Rust:

```bash
rustup update
```

### Performance Baixa

Execute sempre em modo release para melhor performance:

```bash
cargo run --release
```

## Estrutura de Diretórios Esperada

```
pong-rust/
├── assets/
│   ├── fonts/
│   │   └── retro.ttf
│   └── sounds/
├── src/
│   ├── game/
│   │   ├── ball.rs
│   │   ├── game_state.rs
│   │   ├── mod.rs
│   │   └── paddle.rs
│   └── main.rs
├── Cargo.toml
├── README.md
└── INSTALACAO.md
```

## Comandos Úteis do Cargo

```bash
# Compilar o projeto
cargo build

# Compilar em modo release
cargo build --release

# Executar o projeto
cargo run

# Executar em modo release
cargo run --release

# Limpar arquivos compilados
cargo clean

# Verificar o código sem compilar
cargo check

# Formatar o código
cargo fmt

# Verificar problemas no código
cargo clippy
```

## Próximos Passos

Após executar o jogo com sucesso, você pode:

1. Ler o código-fonte comentado em `src/`
2. Modificar parâmetros do jogo (velocidade, tamanho das raquetes, etc)
3. Adicionar novas funcionalidades
4. Estudar os conceitos de Rust aplicados no projeto

## Suporte

Se encontrar problemas, verifique:

1. Versão do Rust (deve ser 1.70+)
2. Instalação correta do SDL2
3. Estrutura de diretórios do projeto
4. Mensagens de erro detalhadas do compilador

O compilador Rust fornece mensagens de erro muito detalhadas que geralmente indicam exatamente o que precisa ser corrigido.
