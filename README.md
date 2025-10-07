# Pong with Rust

Um jogo clássico de Pong implementado em Rust para fins educacionais.

## Descrição

Este é um jogo Pong para 2 jogadores desenvolvido em Rust utilizando a biblioteca SDL2. O projeto foi criado com o objetivo de ensinar conceitos de desenvolvimento de jogos e programação em Rust de forma clara e didática.

## Características

- Jogo para 2 jogadores (multiplayer local)
- Gráficos simples no estilo retro dos anos 80
- Sistema de pontuação (primeiro a 5 pontos vence)
- Controles simples e intuitivos
- Código totalmente comentado em português
- Física de colisão realista
- Aumento progressivo de dificuldade

## Requisitos

### Rust

O projeto requer Rust 1.70 ou superior. Para instalar o Rust, visite [rustup.rs](https://rustup.rs/).

### SDL2

O jogo utiliza a biblioteca SDL2 para gráficos, áudio e entrada. É necessário instalar as bibliotecas de desenvolvimento SDL2.

#### Ubuntu/Debian

```bash
sudo apt-get install libsdl2-dev libsdl2-mixer-dev libsdl2-ttf-dev
```

#### macOS (Homebrew)

```bash
brew install sdl2 sdl2_mixer sdl2_ttf
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

#### Windows (MSVC)

1. Baixe as bibliotecas de desenvolvimento SDL2 de [libsdl.org](https://www.libsdl.org/)
2. Siga as instruções de instalação para Windows no site oficial

## Como Jogar

### Compilar e Executar

```bash
cargo run --release
```

### Controles

**Jogador 1 (Raquete Esquerda - Azul)**
- `W` - Mover para cima
- `S` - Mover para baixo

**Jogador 2 (Raquete Direita - Vermelha)**
- `Seta para Cima` - Mover para cima
- `Seta para Baixo` - Mover para baixo

**Controles Gerais**
- `Espaço` - Iniciar jogo / Pausar / Continuar / Novo jogo
- `ESC` - Voltar ao menu / Sair do jogo

### Regras

1. O objetivo é fazer a bola passar pela raquete do adversário
2. Cada vez que a bola ultrapassa uma raquete, o jogador oposto marca 1 ponto
3. O primeiro jogador a atingir 5 pontos vence a partida
4. A velocidade da bola aumenta ligeiramente a cada rebatida

## Estrutura do Projeto

```
pong-rust/
├── src/
│   ├── main.rs              # Ponto de entrada e loop principal
│   └── game/
│       ├── mod.rs           # Módulo do jogo
│       ├── ball.rs          # Lógica da bola
│       ├── paddle.rs        # Lógica das raquetes
│       └── game_state.rs    # Estado e lógica principal do jogo
├── assets/
│   ├── fonts/
│   │   └── retro.ttf        # Fonte retro para o jogo
│   └── sounds/              # Sons do jogo (vazio por enquanto)
├── Cargo.toml               # Configuração e dependências
└── README.md                # Este arquivo
```

## Conceitos de Rust Demonstrados

Este projeto demonstra diversos conceitos importantes de Rust:

1. **Sistema de Módulos** - Organização do código em módulos (`mod`)
2. **Structs e Implementações** - Estruturas de dados e métodos
3. **Ownership e Borrowing** - Referências mutáveis e imutáveis
4. **Pattern Matching** - Uso de `match` para controle de fluxo
5. **Enums** - Estados do jogo com `GamePhase`
6. **Traits** - Utilização de traits da biblioteca SDL2
7. **Error Handling** - Tratamento de erros com `Result`
8. **FFI (Foreign Function Interface)** - Integração com biblioteca C (SDL2)

## Dependências

- `sdl2` (0.38.0) - Biblioteca para gráficos, áudio e entrada
- `rand` (0.9.2) - Geração de números aleatórios

## Licença

MIT License

## Créditos

- Fonte: Retro Gaming por Daymarius (100% Free)
- Desenvolvido para fins educacionais

## Melhorias Futuras

Possíveis melhorias que podem ser implementadas pelos alunos:

1. Adicionar efeitos sonoros para colisões
2. Implementar música de fundo
3. Adicionar power-ups (bola mais rápida, raquete maior, etc)
4. Criar diferentes níveis de dificuldade
5. Adicionar modo de jogo contra IA
6. Implementar sistema de replay
7. Adicionar partículas visuais nos impactos
8. Criar menu de configurações
