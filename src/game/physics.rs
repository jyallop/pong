use super::Ball;
use super::Paddle;

pub fn check_collision(ball: &Ball, paddle: &Paddle) -> bool {
    check_y(ball, paddle) & check_x(ball, paddle)
}

fn check_y(ball: &Ball, paddle: &Paddle) -> bool {
    //    ball.center.y < paddle.y + paddle.height & ball.center.y > paddle.y
    true
}

fn check_x(ball: &Ball, paddle: &Paddle) -> bool {
    if ball.velocity.x < 0.0 {
        ball.center.x + ball.radius > paddle.x
    } else {
        ball.center.x - ball.radius < paddle.x + paddle.width
    }
}
