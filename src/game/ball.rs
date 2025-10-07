// Estrutura e lógica da bola do jogo

use sdl2::rect::Rect;
use rand::Rng;
use std::f32::consts::PI;

/// Representa a bola no jogo Pong
/// A bola se move pela tela e colide com as raquetes e paredes
pub struct Ball {
    /// Posição X da bola (centro)
    pub x: f32,
    /// Posição Y da bola (centro)
    pub y: f32,
    /// Tamanho da bola (largura e altura são iguais)
    pub size: f32,
    /// Velocidade horizontal da bola (pixels por segundo)
    pub vel_x: f32,
    /// Velocidade vertical da bola (pixels por segundo)
    pub vel_y: f32,
    /// Velocidade base da bola
    base_speed: f32,
}

impl Ball {
    /// Cria uma nova bola na posição especificada
    /// 
    /// # Argumentos
    /// 
    /// * `x` - Posição X inicial
    /// * `y` - Posição Y inicial
    /// * `size` - Tamanho da bola
    /// * `speed` - Velocidade base da bola
    pub fn new(x: f32, y: f32, size: f32, speed: f32) -> Ball {
        Ball {
            x,
            y,
            size,
            vel_x: 0.0,
            vel_y: 0.0,
            base_speed: speed,
        }
    }

    /// Lança a bola em uma direção aleatória
    /// A bola sempre começa indo para a esquerda ou direita com um ângulo aleatório
    pub fn launch(&mut self) {
        let mut rng = rand::rng();
        
        // Escolhe direção aleatória (esquerda ou direita)
        let direction = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
        
        // Ângulo aleatório entre -45 e 45 graus
        let angle = rng.random_range(-PI / 4.0..PI / 4.0);
        
        // Calcula velocidades baseadas no ângulo
        self.vel_x = self.base_speed * angle.cos() * direction;
        self.vel_y = self.base_speed * angle.sin();
    }

    /// Atualiza a posição da bola baseado no tempo decorrido
    /// 
    /// # Argumentos
    /// 
    /// * `delta_time` - Tempo decorrido desde o último frame (em segundos)
    pub fn update(&mut self, delta_time: f32) {
        self.x += self.vel_x * delta_time;
        self.y += self.vel_y * delta_time;
    }

    /// Inverte a direção horizontal da bola (colisão com raquete)
    pub fn bounce_horizontal(&mut self) {
        self.vel_x = -self.vel_x;
        
        // Aumenta ligeiramente a velocidade a cada rebatida para aumentar dificuldade
        self.vel_x *= 1.05;
        self.vel_y *= 1.05;
    }

    /// Inverte a direção vertical da bola (colisão com parede superior/inferior)
    pub fn bounce_vertical(&mut self) {
        self.vel_y = -self.vel_y;
    }

    /// Verifica colisão com as paredes superior e inferior
    /// 
    /// # Argumentos
    /// 
    /// * `screen_height` - Altura da tela
    /// 
    /// # Retorna
    /// 
    /// `true` se houve colisão, `false` caso contrário
    pub fn check_wall_collision(&mut self, screen_height: f32) -> bool {
        let half_size = self.size / 2.0;
        
        // Colisão com parede superior
        if self.y - half_size <= 0.0 {
            self.y = half_size;
            self.bounce_vertical();
            return true;
        }
        
        // Colisão com parede inferior
        if self.y + half_size >= screen_height {
            self.y = screen_height - half_size;
            self.bounce_vertical();
            return true;
        }
        
        false
    }

    /// Retorna um retângulo SDL para renderização
    pub fn get_rect(&self) -> Rect {
        let half_size = self.size / 2.0;
        Rect::new(
            (self.x - half_size) as i32,
            (self.y - half_size) as i32,
            self.size as u32,
            self.size as u32,
        )
    }

    /// Reseta a bola para o centro da tela
    /// 
    /// # Argumentos
    /// 
    /// * `screen_width` - Largura da tela
    /// * `screen_height` - Altura da tela
    pub fn reset(&mut self, screen_width: f32, screen_height: f32) {
        self.x = screen_width / 2.0;
        self.y = screen_height / 2.0;
        self.vel_x = 0.0;
        self.vel_y = 0.0;
    }
}
