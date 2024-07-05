struct Body {
	mass: f64,
	pos_x: f64,
	pos_y: f64,
	vel_x: f64,
	vel_y: f64,
}

fn get_distance(main_body: &Body, other_body: &Body) -> f64 {
	let dist_x2 = (main_body.pos_x-other_body.pos_x).powf(2.0);
	let dist_y2 = (main_body.pos_y-other_body.pos_y).powf(2.0);

	return dist_x2+dist_y2.powf(0.5);
}

fn calculate_x_vdot(main_body: &Body, other_body: &Body) -> f64 {
	const G_CONST:f64 = 6.6743e-11;
	let x_vdot = -(main_body.pos_x - other_body.pos_x) * (G_CONST * other_body.mass) / get_distance(main_body, other_body).powf(3.0);
	return x_vdot;
}

fn calculate_y_vdot(main_body: &Body, other_body: &Body) -> f64 {
	const G_CONST:f64 = 6.6743e-11;
	let y_vdot = -(main_body.pos_y - other_body.pos_y) * (G_CONST * other_body.mass) /  get_distance(main_body, other_body).powf(3.0);
	return y_vdot;
}

fn update_veloecity(main_body: &mut Body, other_body: &Body) {
	// Update the velocity of the body from the influence of another body.
	main_body.vel_x = main_body.vel_x + calculate_x_vdot(main_body, other_body);
	main_body.vel_y = main_body.vel_y + calculate_y_vdot(main_body, other_body);
	//Run this on with all Other Bodies to Get resultant force.
}

fn update_position(main_body: &mut Body, time_step: f64) {
	main_body.pos_x += main_body.vel_x * time_step;
	main_body.pos_y += main_body.vel_y * time_step;
}

fn main() {

	let time_step:f64 = 1.0;

	let number_of_time_steps:u32 = 365 * 60 * 60 * 24;
	let check_in_time:u32 = 60 * 60 * 24;

	let sun = Body{mass: 2e30, pos_x:0.0, pos_y:0.0, vel_x:0.0, vel_y:0.0};
	let mercury = Body{mass:3.3e23, pos_x:0.0, pos_y:57e9, vel_x:47.3e3, vel_y:0.0};
	let venus = Body{mass:4.8e24, pos_x:0.0, pos_y:108e9, vel_x:35e3, vel_y:0.0};
	let earth = Body{mass:6e24, pos_x:0.0, pos_y:-150e9, vel_x:-29782.0, vel_y:0.0};
	let mars = Body{mass:6.4e23, pos_x:0.0, pos_y:227e9, vel_x:24e3, vel_y:0.0};

	let mut all_bodies:[Body;5] = [sun,mercury,venus,earth,mars];

	let mut n_t:u32 = 0;
	while n_t < number_of_time_steps{
		for i in 0..all_bodies.len() {
			let body1_ptr : *mut _ = &mut all_bodies[i];
			for j in 0..all_bodies.len(){
				if i == j {
					continue
				} else {
					unsafe{
						update_veloecity(&mut *body1_ptr,&mut all_bodies[j]);
					}
				}
			}
			update_position(&mut all_bodies[i], 1.0);
		}
		if n_t%check_in_time == 0 {
			println!("Time Passed : {} Days", time_step*(n_t as f64)/(check_in_time as f64));
		}
		n_t+=1
	}

}
