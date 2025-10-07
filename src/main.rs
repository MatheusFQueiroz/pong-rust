// Pong em Rust - Jogo educacional
// Um jogo clássico de Pong para 2 jogadores

mod game;

use game::game_state::{GamePhase, GameState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::time::Duration;

/// Constantes do jogo
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "Pong with Rust";

fn main() -> Result<(), String> {
    // Inicializa o SDL2 (biblioteca para gráficos, áudio e entrada)
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    // Inicializa o mixer de áudio
    let _audio = sdl_context.audio()?;
    sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;
    let _ = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    sdl2::mixer::allocate_channels(4);

    // Cria a janela do jogo
    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Cria o canvas para desenhar (com aceleração de hardware e vsync)
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Cria o estado do jogo
    let mut game_state = GameState::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    // Carrega a fonte para texto
    let font = ttf_context
        .load_font("assets/fonts/retro.ttf", 32)
        .map_err(|e| e.to_string())?;

    // Gerenciador de eventos (teclado, mouse, etc)
    let mut event_pump = sdl_context.event_pump()?;

    // Variáveis para controle de tempo
    let mut last_time = std::time::Instant::now();

    // Loop principal do jogo
    'running: loop {
        // Calcula o tempo decorrido desde o último frame (delta time)
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Processa eventos (teclado, mouse, fechar janela)
        for event in event_pump.poll_iter() {
            match event {
                // Evento de fechar janela
                Event::Quit { .. } => break 'running,

                // Eventos de teclado
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    handle_keydown(&mut game_state, keycode);
                }

                _ => {}
            }
        }

        // Obtém o estado atual do teclado para controle contínuo
        let keyboard_state = event_pump.keyboard_state();

        // Controla raquete esquerda (Jogador 1) com W e S
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            game_state.paddle_left.move_up(delta_time);
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            game_state
                .paddle_left
                .move_down(delta_time, game_state.screen_height);
        }

        // Controla raquete direita (Jogador 2) com setas para cima e para baixo
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
            game_state.paddle_right.move_up(delta_time);
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
            game_state
                .paddle_right
                .move_down(delta_time, game_state.screen_height);
        }

        // Atualiza o estado do jogo
        game_state.update(delta_time);

        // Renderiza o jogo baseado na fase atual
        match game_state.phase {
            GamePhase::Menu => {
                render_menu(&mut canvas, &font, &game_state)?;
            }
            GamePhase::Playing | GamePhase::Paused => {
                game_state.render(&mut canvas);
                render_score(&mut canvas, &font, &game_state)?;

                if game_state.phase == GamePhase::Paused {
                    render_pause(&mut canvas, &font)?;
                }
            }
            GamePhase::GameOver => {
                game_state.render(&mut canvas);
                render_score(&mut canvas, &font, &game_state)?;
                render_game_over(&mut canvas, &font, &game_state)?;
            }
        }

        // Apresenta o frame renderizado na tela
        canvas.present();

        // Pequena pausa para não sobrecarregar a CPU
        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

/// Trata eventos de teclas pressionadas
///
/// # Argumentos
///
/// * `game_state` - Estado atual do jogo
/// * `keycode` - Código da tecla pressionada
fn handle_keydown(game_state: &mut GameState, keycode: Keycode) {
    match keycode {
        // ESC para sair ou voltar ao menu
        Keycode::Escape => {
            if game_state.phase == GamePhase::Menu {
                std::process::exit(0);
            } else {
                game_state.reset();
            }
        }

        // ESPAÇO para iniciar/pausar/reiniciar
        Keycode::Space => match game_state.phase {
            GamePhase::Menu => {
                game_state.start_round();
            }
            GamePhase::Playing => {
                game_state.phase = GamePhase::Paused;
            }
            GamePhase::Paused => {
                game_state.phase = GamePhase::Playing;
            }
            GamePhase::GameOver => {
                game_state.reset();
            }
        },

        _ => {}
    }
}

/// Renderiza o menu inicial
///
/// # Argumentos
///
/// * `canvas` - Canvas SDL para desenhar
/// * `font` - Fonte para renderizar texto
/// * `game_state` - Estado atual do jogo
fn render_menu(
    canvas: &mut Canvas<Window>,
    font: &Font,
    game_state: &GameState,
) -> Result<(), String> {
    // Limpa a tela
    canvas.set_draw_color(game_state.background_color);
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    // Título do jogo
    render_text(
        canvas,
        &texture_creator,
        font,
        "PONG WITH RUST",
        SCREEN_WIDTH as i32 / 2,
        100,
        Color::RGB(100, 200, 255),
    )?;

    // Instruções
    render_text(
        canvas,
        &texture_creator,
        font,
        "JOGADOR 1: W / S",
        SCREEN_WIDTH as i32 / 2,
        250,
        Color::RGB(200, 200, 200),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "JOGADOR 2: SETAS",
        SCREEN_WIDTH as i32 / 2,
        300,
        Color::RGB(200, 200, 200),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "PRIMEIRO A 5 PONTOS VENCE",
        SCREEN_WIDTH as i32 / 2,
        370,
        Color::RGB(255, 255, 100),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "PRESSIONE ESPACO PARA INICIAR",
        SCREEN_WIDTH as i32 / 2,
        450,
        Color::RGB(100, 255, 100),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "ESC PARA SAIR",
        SCREEN_WIDTH as i32 / 2,
        500,
        Color::RGB(150, 150, 150),
    )?;

    Ok(())
}

/// Renderiza a pontuação
///
/// # Argumentos
///
/// * `canvas` - Canvas SDL para desenhar
/// * `font` - Fonte para renderizar texto
/// * `game_state` - Estado atual do jogo
fn render_score(
    canvas: &mut Canvas<Window>,
    font: &Font,
    game_state: &GameState,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    // Pontuação do jogador 1 (esquerda)
    let score_left_text = format!("{}", game_state.score_left);
    render_text(
        canvas,
        &texture_creator,
        font,
        &score_left_text,
        SCREEN_WIDTH as i32 / 4,
        30,
        Color::RGB(100, 200, 255),
    )?;

    // Pontuação do jogador 2 (direita)
    let score_right_text = format!("{}", game_state.score_right);
    render_text(
        canvas,
        &texture_creator,
        font,
        &score_right_text,
        3 * SCREEN_WIDTH as i32 / 4,
        30,
        Color::RGB(255, 100, 100),
    )?;

    Ok(())
}

/// Renderiza a tela de pausa
///
/// # Argumentos
///
/// * `canvas` - Canvas SDL para desenhar
/// * `font` - Fonte para renderizar texto
fn render_pause(canvas: &mut Canvas<Window>, font: &Font) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    render_text(
        canvas,
        &texture_creator,
        font,
        "PAUSADO",
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2 - 20,
        Color::RGB(255, 255, 100),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "ESPACO PARA CONTINUAR",
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2 + 30,
        Color::RGB(200, 200, 200),
    )?;

    Ok(())
}

/// Renderiza a tela de fim de jogo
///
/// # Argumentos
///
/// * `canvas` - Canvas SDL para desenhar
/// * `font` - Fonte para renderizar texto
/// * `game_state` - Estado atual do jogo
fn render_game_over(
    canvas: &mut Canvas<Window>,
    font: &Font,
    game_state: &GameState,
) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();

    // Determina o vencedor
    let winner_text = if game_state.score_left >= game_state.max_score {
        "JOGADOR 1 VENCEU!"
    } else {
        "JOGADOR 2 VENCEU!"
    };

    let winner_color = if game_state.score_left >= game_state.max_score {
        Color::RGB(100, 200, 255)
    } else {
        Color::RGB(255, 100, 100)
    };

    render_text(
        canvas,
        &texture_creator,
        font,
        winner_text,
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2 - 40,
        winner_color,
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "ESPACO PARA NOVO JOGO",
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2 + 20,
        Color::RGB(200, 200, 200),
    )?;

    render_text(
        canvas,
        &texture_creator,
        font,
        "ESC PARA MENU",
        SCREEN_WIDTH as i32 / 2,
        SCREEN_HEIGHT as i32 / 2 + 60,
        Color::RGB(150, 150, 150),
    )?;

    Ok(())
}

/// Função auxiliar para renderizar texto centralizado
///
/// # Argumentos
///
/// * `canvas` - Canvas SDL para desenhar
/// * `texture_creator` - Criador de texturas SDL
/// * `font` - Fonte para renderizar texto
/// * `text` - Texto a ser renderizado
/// * `x` - Posição X do centro do texto
/// * `y` - Posição Y do topo do texto
/// * `color` - Cor do texto
fn render_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), String> {
    // Cria uma superfície com o texto renderizado
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;

    // Converte a superfície em textura
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    // Obtém as dimensões do texto
    let text_query = texture.query();
    let text_width = text_query.width;
    let text_height = text_query.height;

    // Calcula a posição para centralizar o texto
    let target = Rect::new(x - text_width as i32 / 2, y, text_width, text_height);

    // Desenha o texto na tela
    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}
