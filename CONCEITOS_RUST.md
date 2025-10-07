# Conceitos de Rust Demonstrados no Projeto

Este documento explica os principais conceitos de Rust aplicados no projeto Pong, servindo como guia de estudo para os alunos.

## 1. Sistema de Módulos

### Localização
- `src/main.rs` - linha 3: `mod game;`
- `src/game/mod.rs` - declaração de submódulos

### Conceito
O Rust organiza código em módulos hierárquicos. No projeto:
- `game` é o módulo principal do jogo
- `ball`, `paddle` e `game_state` são submódulos

### Exemplo
```rust
mod game;  // Declara o módulo game
use game::game_state::GameState;  // Importa um tipo específico
```

## 2. Structs e Implementações

### Localização
- `src/game/ball.rs` - linhas 8-18: struct Ball
- `src/game/ball.rs` - linhas 20-140: impl Ball

### Conceito
Structs definem tipos de dados personalizados. Implementações (`impl`) adicionam métodos a esses tipos.

### Exemplo
```rust
pub struct Ball {
    pub x: f32,
    pub y: f32,
    // ... outros campos
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Ball {
        Ball { x, y, /* ... */ }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vel_x * delta_time;
    }
}
```

### Pontos de Ensino
- `pub` torna campos e métodos públicos
- `&mut self` permite modificar o objeto
- `&self` permite apenas leitura

## 3. Ownership e Borrowing

### Localização
- `src/main.rs` - linha 47: `let mut game_state = ...`
- `src/main.rs` - linha 70: `handle_keydown(&mut game_state, keycode)`

### Conceito
O sistema de ownership do Rust garante segurança de memória sem garbage collector.

### Regras
1. Cada valor tem um único dono
2. Quando o dono sai de escopo, o valor é destruído
3. Referências permitem emprestar valores sem transferir ownership

### Exemplo
```rust
let mut game_state = GameState::new(...);  // game_state é o dono

// Empresta mutavelmente para modificar
handle_keydown(&mut game_state, keycode);

// Empresta imutuavelmente para ler
render_score(&canvas, &font, &game_state);
```

### Pontos de Ensino
- `&` cria uma referência imutável (pode ter múltiplas)
- `&mut` cria uma referência mutável (apenas uma por vez)
- Isso previne data races em tempo de compilação

## 4. Pattern Matching

### Localização
- `src/main.rs` - linhas 58-72: match em eventos
- `src/main.rs` - linhas 138-162: match em keycode

### Conceito
`match` é uma estrutura de controle poderosa que verifica todos os casos possíveis.

### Exemplo
```rust
match event {
    Event::Quit { .. } => break 'running,
    Event::KeyDown { keycode: Some(keycode), .. } => {
        handle_keydown(&mut game_state, keycode);
    }
    _ => {}  // Caso padrão para outros eventos
}
```

### Pontos de Ensino
- O compilador garante que todos os casos são tratados
- `..` ignora campos não utilizados
- `_` é o padrão "coringa" que captura tudo

## 5. Enums

### Localização
- `src/game/game_state.rs` - linhas 8-16: enum GamePhase

### Conceito
Enums definem um tipo que pode ser uma de várias variantes.

### Exemplo
```rust
#[derive(PartialEq, Clone, Copy)]
pub enum GamePhase {
    Menu,
    Playing,
    Paused,
    GameOver,
}
```

### Uso
```rust
if game_state.phase == GamePhase::Playing {
    // Lógica do jogo
}

match game_state.phase {
    GamePhase::Menu => render_menu(...),
    GamePhase::Playing => game_state.render(...),
    // ...
}
```

### Pontos de Ensino
- `#[derive(...)]` gera automaticamente implementações de traits
- `PartialEq` permite comparações com `==`
- `Clone` permite copiar o valor
- `Copy` permite copiar implicitamente (para tipos simples)

## 6. Traits

### Localização
Usados implicitamente ao trabalhar com SDL2

### Conceito
Traits definem comportamentos compartilhados (similar a interfaces).

### Exemplo no Projeto
```rust
// O tipo Canvas implementa vários traits
canvas.set_draw_color(color);  // Trait para definir cor
canvas.clear();                 // Trait para limpar
canvas.present();               // Trait para apresentar
```

### Pontos de Ensino
- Traits permitem polimorfismo
- Muitas funcionalidades em Rust são baseadas em traits
- Traits podem ser derivados automaticamente com `#[derive]`

## 7. Error Handling

### Localização
- `src/main.rs` - linha 21: `fn main() -> Result<(), String>`
- `src/main.rs` - linha 24: `let sdl_context = sdl2::init()?;`

### Conceito
Rust usa `Result<T, E>` para tratamento de erros explícito.

### Exemplo
```rust
fn main() -> Result<(), String> {
    // O operador ? propaga erros automaticamente
    let sdl_context = sdl2::init()?;
    let window = video_subsystem.window(...).build()?;
    
    // Se tudo correr bem, retorna Ok
    Ok(())
}
```

### Pontos de Ensino
- `Result<T, E>` pode ser `Ok(valor)` ou `Err(erro)`
- O operador `?` retorna o erro se houver, ou desempacota o valor
- Força o programador a lidar com possíveis falhas

## 8. Lifetimes (Implícitos)

### Localização
- `src/main.rs` - linha 48: `let font = ttf_context.load_font(...)`

### Conceito
Lifetimes garantem que referências são sempre válidas.

### Exemplo
```rust
let ttf_context = sdl2::ttf::init()?;
let font = ttf_context.load_font("assets/fonts/retro.ttf", 32)?;

// font não pode sobreviver a ttf_context
// O compilador garante isso automaticamente
```

### Pontos de Ensino
- Lifetimes previnem dangling pointers
- Muitas vezes são inferidos automaticamente
- Garantem segurança de memória em tempo de compilação

## 9. Mutabilidade Explícita

### Localização
- `src/main.rs` - linha 47: `let mut game_state`
- `src/game/ball.rs` - linha 67: `pub fn update(&mut self, ...)`

### Conceito
Por padrão, variáveis são imutáveis. Mutabilidade deve ser explícita.

### Exemplo
```rust
let x = 5;      // Imutável
// x = 6;       // ERRO: não pode modificar

let mut y = 5;  // Mutável
y = 6;          // OK
```

### Pontos de Ensino
- Imutabilidade por padrão previne bugs
- `mut` sinaliza claramente onde mudanças ocorrem
- Facilita raciocínio sobre o código

## 10. Type Safety

### Localização
Todo o código demonstra type safety

### Conceito
O Rust é fortemente tipado e verifica tipos em tempo de compilação.

### Exemplo
```rust
let screen_width: f32 = 800.0;  // Tipo explícito
let screen_height = 600.0;      // Tipo inferido como f32

// Conversões devem ser explícitas
let width_int = screen_width as i32;
```

### Pontos de Ensino
- Tipos previnem muitos bugs em tempo de compilação
- Conversões explícitas evitam perda acidental de dados
- O compilador infere tipos quando possível

## 11. Constantes

### Localização
- `src/main.rs` - linhas 18-20: constantes do jogo

### Conceito
Constantes são valores imutáveis conhecidos em tempo de compilação.

### Exemplo
```rust
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "Pong with Rust";
```

### Pontos de Ensino
- `const` é avaliado em tempo de compilação
- Tipos devem ser explícitos em constantes
- Convenção: nomes em UPPER_SNAKE_CASE

## 12. Closures (Implícito)

### Localização
- `src/main.rs` - linha 56: `for event in event_pump.poll_iter()`

### Conceito
Closures são funções anônimas que podem capturar seu ambiente.

### Exemplo Simples
```rust
let multiplicador = 2;
let dobrar = |x| x * multiplicador;  // Closure que captura multiplicador
println!("{}", dobrar(5));  // Imprime 10
```

## Exercícios Sugeridos para Alunos

1. **Ownership**: Tente mover `game_state` para uma função e usá-lo depois. O que acontece?

2. **Borrowing**: Modifique uma função para receber `&Ball` em vez de `&mut Ball`. Quais métodos ainda funcionam?

3. **Pattern Matching**: Adicione um novo tipo de evento (ex: mouse) e trate-o no match.

4. **Enums**: Adicione uma nova fase ao jogo (ex: `Tutorial`) e implemente sua lógica.

5. **Error Handling**: Modifique o código para tratar erros de forma mais específica em vez de usar `String`.

6. **Structs**: Crie uma nova struct `PowerUp` com posição e tipo, e implemente métodos para ela.

7. **Traits**: Crie um trait `Drawable` e implemente-o para `Ball` e `Paddle`.

8. **Mutabilidade**: Identifique todas as variáveis mutáveis. Alguma poderia ser imutável?

## Recursos Adicionais

- [The Rust Book](https://doc.rust-lang.org/book/) - Livro oficial do Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Aprenda com exemplos
- [Rustlings](https://github.com/rust-lang/rustlings) - Exercícios práticos
- [SDL2 Rust Docs](https://docs.rs/sdl2/) - Documentação da biblioteca SDL2
