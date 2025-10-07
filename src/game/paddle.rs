// Estrutura e lógica das raquetes do jogo

use sdl2::rect::Rect;

/// Representa uma raquete no jogo Pong
/// Cada jogador controla uma raquete para rebater a bola
pub struct Paddle {
    /// Posição X da raquete (canto superior esquerdo)
    pub x: f32,
    /// Posição Y da raquete (canto superior esquerdo)
    pub y: f32,
    /// Largura da raquete
    pub width: f32,
    /// Altura da raquete
    pub height: f32,
    /// Velocidade de movimento da raquete (pixels por segundo)
    pub speed: f32,
}

impl Paddle {
    /// Cria uma nova raquete
    /// 
    /// # Argumentos
    /// 
    /// * `x` - Posição X inicial
    /// * `y` - Posição Y inicial
    /// * `width` - Largura da raquete
    /// * `height` - Altura da raquete
    /// * `speed` - Velocidade de movimento
    pub fn new(x: f32, y: f32, width: f32, height: f32, speed: f32) -> Paddle {
        Paddle {
            x,
            y,
            width,
            height,
            speed,
        }
    }

    /// Move a raquete para cima
    /// 
    /// # Argumentos
    /// 
    /// * `delta_time` - Tempo decorrido desde o último frame (em segundos)
    pub fn move_up(&mut self, delta_time: f32) {
        self.y -= self.speed * delta_time;
        
        // Limita a raquete para não sair da tela (topo)
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    /// Move a raquete para baixo
    /// 
    /// # Argumentos
    /// 
    /// * `delta_time` - Tempo decorrido desde o último frame (em segundos)
    /// * `screen_height` - Altura da tela
    pub fn move_down(&mut self, delta_time: f32, screen_height: f32) {
        self.y += self.speed * delta_time;
        
        // Limita a raquete para não sair da tela (fundo)
        if self.y + self.height > screen_height {
            self.y = screen_height - self.height;
        }
    }

    /// Move a raquete para uma posição Y específica (usado para controle por mouse)
    /// 
    /// # Argumentos
    /// 
    /// * `target_y` - Posição Y alvo (centro da raquete)
    /// * `screen_height` - Altura da tela
    //  pub fn move_to(&mut self, target_y: f32, screen_height: f32) {
    //      // Centraliza a raquete na posição do mouse
    //      self.y = target_y - self.height / 2.0;
        
    //      // Limita a raquete para não sair da tela
    //      if self.y < 0.0 {
    //          self.y = 0.0;
    //      }
    //      if self.y + self.height > screen_height {
    //          self.y = screen_height - self.height;
    //      }
    //  }

    /// Verifica colisão com a bola
    /// 
    /// # Argumentos
    /// 
    /// * `ball_x` - Posição X da bola
    /// * `ball_y` - Posição Y da bola
    /// * `ball_size` - Tamanho da bola
    /// 
    /// # Retorna
    /// 
    /// `true` se houve colisão, `false` caso contrário
    pub fn check_collision(&self, ball_x: f32, ball_y: f32, ball_size: f32) -> bool {
        let ball_half_size = ball_size / 2.0;
        
        // Verifica se a bola está dentro dos limites da raquete
        ball_x + ball_half_size >= self.x
            && ball_x - ball_half_size <= self.x + self.width
            && ball_y + ball_half_size >= self.y
            && ball_y - ball_half_size <= self.y + self.height
    }

    /// Retorna um retângulo SDL para renderização
    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.x as i32,
            self.y as i32,
            self.width as u32,
            self.height as u32,
        )
    }
}
