use model::game::SnakeGame;

pub trait SnakeDisplay {
    fn render(&mut self, snake_game: &SnakeGame);
}