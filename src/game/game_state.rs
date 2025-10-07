// Estado do jogo e lógica principal

use super::ball::Ball;
use super::paddle::Paddle;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Estados possíveis do jogo
#[derive(PartialEq, Clone, Copy)]
pub enum GamePhase {
    /// Tela inicial com instruções
    Menu,
    /// Jogo em andamento
    Playing,
    /// Jogo pausado
    Paused,
    /// Fim de jogo (alguém ganhou)
    GameOver,
}

/// Estrutura principal que gerencia todo o estado do jogo
pub struct GameState {
    /// Largura da tela
    pub screen_width: f32,
    /// Altura da tela
    pub screen_height: f32,
    /// Cor de fundo da tela
    pub background_color: Color,
    /// Fase atual do jogo
    pub phase: GamePhase,
    /// Bola do jogo
    pub ball: Ball,
    /// Raquete do jogador 1 (esquerda)
    pub paddle_left: Paddle,
    /// Raquete do jogador 2 (direita)
    pub paddle_right: Paddle,
    /// Pontuação do jogador 1
    pub score_left: u32,
    /// Pontuação do jogador 2
    pub score_right: u32,
    /// Pontuação máxima para vencer
    pub max_score: u32,
}

impl GameState {
    /// Cria um novo estado de jogo
    /// 
    /// # Argumentos
    /// 
    /// * `screen_width` - Largura da tela
    /// * `screen_height` - Altura da tela
    pub fn new(screen_width: f32, screen_height: f32) -> GameState {
        // Configurações das raquetes
        let paddle_width = 15.0;
        let paddle_height = 80.0;
        let paddle_offset = 30.0;
        let paddle_speed = 400.0;
        
        // Cria raquete esquerda (jogador 1)
        let paddle_left = Paddle::new(
            paddle_offset,
            (screen_height - paddle_height) / 2.0,
            paddle_width,
            paddle_height,
            paddle_speed,
        );
        
        // Cria raquete direita (jogador 2)
        let paddle_right = Paddle::new(
            screen_width - paddle_offset - paddle_width,
            (screen_height - paddle_height) / 2.0,
            paddle_width,
            paddle_height,
            paddle_speed,
        );
        
        // Cria a bola
        let ball = Ball::new(
            screen_width / 2.0,
            screen_height / 2.0,
            12.0,
            300.0,
        );
        
        GameState {
            screen_width,
            screen_height,
            background_color: Color::RGB(20, 20, 30),
            phase: GamePhase::Menu,
            ball,
            paddle_left,
            paddle_right,
            score_left: 0,
            score_right: 0,
            max_score: 5,
        }
    }

    /// Inicia uma nova rodada
    pub fn start_round(&mut self) {
        self.ball.reset(self.screen_width, self.screen_height);
        self.ball.launch();
        self.phase = GamePhase::Playing;
    }

    /// Atualiza o estado do jogo
    /// 
    /// # Argumentos
    /// 
    /// * `delta_time` - Tempo decorrido desde o último frame (em segundos)
    pub fn update(&mut self, delta_time: f32) {
        if self.phase != GamePhase::Playing {
            return;
        }

        // Atualiza posição da bola
        self.ball.update(delta_time);

        // Verifica colisão com paredes superior e inferior
        self.ball.check_wall_collision(self.screen_height);

        // Verifica colisão com raquete esquerda
        if self.paddle_left.check_collision(self.ball.x, self.ball.y, self.ball.size) {
            self.ball.bounce_horizontal();
            // Ajusta posição para evitar que a bola fique presa
            self.ball.x = self.paddle_left.x + self.paddle_left.width + self.ball.size / 2.0;
        }

        // Verifica colisão com raquete direita
        if self.paddle_right.check_collision(self.ball.x, self.ball.y, self.ball.size) {
            self.ball.bounce_horizontal();
            // Ajusta posição para evitar que a bola fique presa
            self.ball.x = self.paddle_right.x - self.ball.size / 2.0;
        }

        // Verifica se a bola saiu da tela (ponto marcado)
        if self.ball.x < 0.0 {
            // Jogador 2 marcou ponto
            self.score_right += 1;
            self.check_game_over();
            if self.phase != GamePhase::GameOver {
                self.start_round();
            }
        } else if self.ball.x > self.screen_width {
            // Jogador 1 marcou ponto
            self.score_left += 1;
            self.check_game_over();
            if self.phase != GamePhase::GameOver {
                self.start_round();
            }
        }
    }

    /// Verifica se o jogo terminou (alguém atingiu a pontuação máxima)
    fn check_game_over(&mut self) {
        if self.score_left >= self.max_score || self.score_right >= self.max_score {
            self.phase = GamePhase::GameOver;
        }
    }

    /// Renderiza todos os elementos do jogo
    /// 
    /// # Argumentos
    /// 
    /// * `canvas` - Canvas SDL para desenhar
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // Limpa a tela com a cor de fundo
        canvas.set_draw_color(self.background_color);
        canvas.clear();

        // Desenha linha central (estilo anos 80)
        self.draw_center_line(canvas);

        // Desenha raquetes
        canvas.set_draw_color(Color::RGB(100, 200, 255)); // Azul claro
        canvas.fill_rect(self.paddle_left.get_rect()).unwrap();

        canvas.set_draw_color(Color::RGB(255, 100, 100)); // Vermelho claro
        canvas.fill_rect(self.paddle_right.get_rect()).unwrap();

        // Desenha bola
        canvas.set_draw_color(Color::RGB(255, 255, 100)); // Amarelo
        canvas.fill_rect(self.ball.get_rect()).unwrap();
    }

    /// Desenha a linha central pontilhada
    /// 
    /// # Argumentos
    /// 
    /// * `canvas` - Canvas SDL para desenhar
    fn draw_center_line(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(80, 80, 90));
        
        let center_x = (self.screen_width / 2.0) as i32;
        let dash_height = 15;
        let dash_gap = 10;
        let dash_width = 3;
        
        let mut y = 0;
        while y < self.screen_height as i32 {
            let rect = Rect::new(center_x - dash_width / 2, y, dash_width as u32, dash_height as u32);
            canvas.fill_rect(rect).unwrap();
            y += dash_height + dash_gap;
        }
    }

    /// Reseta o jogo para o estado inicial
    pub fn reset(&mut self) {
        self.score_left = 0;
        self.score_right = 0;
        self.ball.reset(self.screen_width, self.screen_height);
        self.phase = GamePhase::Menu;
    }
}
