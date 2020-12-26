pub fn perceive_vision(mut query: Query<&mut VisionPerception>) {
    for mut vision in query.iter_mut() {
        vision.0.put(0.0);
        vision.0.put_multiple([1.2, 5.0, -1.0]);
    }
}

pub fn perceive_body(mut query: Query<&mut BodyPerception>) {
    for mut body in query.iter_mut() {
        body.0.put(0.0);
        body.0.put_multiple([1.2, 5.0, -1.0]);
    }
}

pub fn decide_movement(
    mut query: Query<&mut MovementDecision>
) {
    for mut movement in query.iter_mut() {
        movement.0.take_multiple(5);
    }
}

pub fn decide_reproduction(
    mut query: Query<&mut ReproductionDecision>
) {
    for mut reproduction in query.iter_mut() {
        reproduction.0.take();
        reproduction.0.take();
    }
}
