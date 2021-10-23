use super::Ball;
use super::Paddle;

pub fn check_wall_collision(ball: &Ball) -> bool {
    (ball.center.y + ball.radius > 1.0) | (ball.center.y - ball.radius < 0.0)
}

pub fn check_collision(ball: &Ball, paddle: &Paddle) -> bool {
    check_y(ball, paddle) & check_x(ball, paddle)
}

fn check_y(ball: &Ball, paddle: &Paddle) -> bool {
    (ball.center.y < paddle.y + paddle.height) & (ball.center.y > paddle.y)
}

fn check_x(ball: &Ball, paddle: &Paddle) -> bool {
    if ball.velocity.x > 0.0 {
        (ball.center.x + ball.radius > paddle.x) & (ball.center.x + ball.radius < paddle.x + paddle.width)
    } else {
        (ball.center.x - ball.radius < paddle.x + paddle.width) & (ball.center.x - ball.radius > paddle.x)
    }
}
