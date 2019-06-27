use std::ops::{Mul, Add, Sub, Div};
use std::error::Error;
use std::f32;
use std::f32::consts::PI;

static mut CURRENT_STATE: States = States{current: 1};

fn main() -> Result<(), Box<dyn Error>> {
    rlbot::run_bot(MyBot { player_index: 0 })
}

struct MyBot {
    player_index: usize,
}

impl rlbot::Bot for MyBot {
    fn set_player_index(&mut self, index: usize) {
        self.player_index = index;
    }

    fn tick(&mut self, packet: &rlbot::GameTickPacket) -> rlbot::ControllerState {
        get_input(self.player_index, packet).unwrap_or_default()
    }
}

fn get_input(player_index: usize, packet: &rlbot::GameTickPacket) -> Option<rlbot::ControllerState> {
	let ball = packet.ball.as_ref()?;
    let mycontrollerstate = packetConverter(player_index, packet, ball);

    Some(rlbot::ControllerState {
        throttle: mycontrollerstate.throttle,
        steer: mycontrollerstate.steer,
        pitch: mycontrollerstate.pitch,
        yaw: mycontrollerstate.yaw,
        roll: mycontrollerstate.roll,
        jump: mycontrollerstate.jump,
        boost: mycontrollerstate.boost,
        handbrake: mycontrollerstate.drift,
    })
}

/* Converts packets into my own packet and controller system and back */
fn packetConverter(player_index: usize, packet: &rlbot::GameTickPacket, ball: &rlbot::BallInfo) -> Controller {
	let evan = &packet.players[player_index];
	let mut opp_index = 1;
	if player_index == 0 {
		opp_index = 1;
	} else {
		opp_index = 0;
	}
	let opp = &packet.players[opp_index];
	let mut mypacket = Packet {
		ballLocation: VectorC {x: ball.physics.location.x, y: ball.physics.location.y, z: ball.physics.location.z, },
		ballVelocity: VectorC {x: ball.physics.velocity.x, y: ball.physics.velocity.y, z: ball.physics.velocity.z, },
		roundActive: packet.game_info.is_round_active,
		evan: PacketPlayer {
			location: VectorC {x: evan.physics.location.x, y: evan.physics.location.y, z: evan.physics.location.z, },
			velocity: VectorC {x: evan.physics.velocity.x, y: evan.physics.velocity.y, z: evan.physics.velocity.z, },
			rotation: RotatorC {pitch: evan.physics.rotation.pitch, yaw: evan.physics.rotation.yaw-(PI/2.0), roll: evan.physics.rotation.roll, },
			isTouching: evan.has_wheel_contact,
			boost: evan.boost,
			team: teamtobool(evan.team),
		},
		opponent: PacketPlayer {
			location: VectorC {x: opp.physics.location.x, y: opp.physics.location.y, z: opp.physics.location.z, },
			velocity: VectorC {x: opp.physics.velocity.x, y: opp.physics.velocity.y, z: opp.physics.velocity.z, },
			rotation: RotatorC {pitch: opp.physics.rotation.pitch, yaw: opp.physics.rotation.yaw, roll: opp.physics.rotation.roll, },
			isTouching: opp.has_wheel_contact,
			boost: opp.boost,
			team: teamtobool(opp.team),
		},
	};
	evan_input(mypacket)
}

fn teamtobool(team: i32) -> bool {
	if team == 0 {
		return true
	}
	false
}

fn evan_input(packet: Packet) -> Controller {
	// Prediction not used yet. Still needs bounces.
	let prediction = Kinematics {
		position: packet.ballLocation,
		velocity: packet.ballVelocity,
		acceleration: VectorC {x: 0.0, y: 0.0, z: -650.0},
		time: 0.0,
		baseUnitName: "Unit".to_string(),
	};
	// Unsafe because global variable current state is being edited by functions.
	unsafe {
		if CURRENT_STATE.expired(&packet) == true {
			// Requires let stateState = States{current: #}; before if statement.
			let kickoffState = States{current: 1};
			if kickoffState.available(&packet) == true {
				CURRENT_STATE = States{current: 1};
			} else {
				CURRENT_STATE = States{current: 2};
			}
		}
		return controllercap(CURRENT_STATE.execute(&packet))
	}
}

/* Controller cap function to keep all non boolian values between -1 and 1 */
fn controllercap(controller_state: Controller) -> Controller {
	Controller {
		throttle: cap(controller_state.throttle),
		boost: controller_state.boost,
		steer: cap(controller_state.steer),
		jump: controller_state.jump,
		pitch: cap(controller_state.pitch),
		yaw: cap(controller_state.yaw),
		roll: cap(controller_state.roll),
		drift: controller_state.drift,
	}
}

fn cap(num: f32) -> f32 {
	if num > 1.0 {
		return 1.0;
	} else if num < -1.0 {
		return -1.0
	}
	num
}

#[derive(Debug)]
struct States {
	current: u32
}
impl States {
	fn available(&self, pack: &Packet) -> bool {
		let curr = &self.current;
		let kickoff: u32 = 1;
		if curr == &kickoff {
			if pack.roundActive == false {
				return true; // Checks if the round is active and returns a true or false.
			}
			return false;
		} else {
			return true; // Always returns true for attb since it is a catch all state.
		}
	}
	fn execute(&self, pack: &Packet) -> Controller {
		let curr = &self.current;
		let kickoff: u32 = 1;
		if curr == &kickoff {
			return executeKickoff(pack);
		} else {
			return executeAttb(pack);
		}
	}
	fn expired(&self, pack: &Packet) -> bool {
		let curr = &self.current;
		let kickoff: u32 = 1;
		if curr == &kickoff {
			if pack.ballLocation.x != 0.0 || pack.ballLocation.y != 0.0 {
			 	return true;
			}
			return false;
		} else {
			return true;
		}
	}
}

fn executeKickoff(pack: &Packet) -> Controller {
	let localBall = toLocal(pack.ballLocation, pack);
	let localDistance = ((localBall.x).powf(2.0) + (localBall.y).powf(2.0)).sqrt();
	let mut controllerBoost = true;
	let mut controllerSteer = steerc(localBall.y.atan2(localBall.x));
	let mut controllerJump = false;
	let mut controllerPitch = 0.0;
	let mut controllerYaw = 0.0;
	let mut controllerRoll = 0.0;
	if pack.evan.location.y.abs() < 700.0 {
		controllerJump = true;
		controllerPitch = -1.0;
	}
	return Controller {
		throttle: 1.0,
		boost: controllerBoost,
		steer: controllerSteer,
		jump: controllerJump,
		pitch: controllerPitch,
		yaw: controllerYaw,
		roll: controllerRoll,
		drift: false,
	};
}
fn executeAttb(pack: &Packet) -> Controller {
	let localBall = toLocal(pack.ballLocation, pack);
	let controllerSteer = steerc(localBall.y.atan2(localBall.x));
	return Controller {
		throttle: 1.0,
		boost: false,
		steer: controllerSteer,
		jump: false,
		pitch: 0.0,
		yaw: 0.0,
		roll: 0.0,
		drift: false,
	};
}

fn steerc(angle: f32) -> f32 {
	angle * -1.0
}

fn toLocal(orig: VectorC, pack: &Packet) -> VectorC {
	let mut adjusted_yaw = pack.evan.rotation.yaw;
	loop {
		if adjusted_yaw > PI {
			adjusted_yaw -= 2.0 * PI;
		} else if adjusted_yaw < -PI {
			adjusted_yaw += 2.0 * PI;
		} else {
			break;
		}
	}
	let new = VectorC {
		x: orig.x - pack.evan.location.x,
		y: orig.y - pack.evan.location.y,
		z: orig.z,
	};
	let angle_of_rotation = adjusted_yaw;
	let angle_to_vector = new.y.atan2(new.x) - (PI/2.0);
	// You can use subtraction to find the new angle the vector needs to be at.
	let newangle = angle_to_vector - angle_of_rotation;
	println!("Yaw Degrees: {} | Yaw Radians: {}", (angle_to_vector*180.0)/PI, angle_to_vector);
	// The magnitude of the vector will stay the same no matter what you rotated it at.
	// To find the x and y coordinates you can just use sin and cos with the magnitude and the angle.
	VectorC {
		x: new.magnitude() * newangle.cos(),
		y: new.magnitude() * newangle.sin(),
		z: new.z,
	}
}

/* CUSTOM CONTROLLER AND PACKET */
#[derive(Debug)]
struct Controller {
	throttle: f32,
	boost: bool,
	steer: f32,
	jump: bool,
	pitch: f32,
	yaw: f32,
	roll: f32,
	drift: bool,
}

#[derive(Debug)]
struct Packet {
	ballLocation: VectorC,
	ballVelocity: VectorC,
	roundActive: bool,
	evan: PacketPlayer,
	opponent: PacketPlayer,
}

#[derive(Debug)]
struct PacketPlayer {
	location: VectorC,
	velocity: VectorC,
	rotation: RotatorC,
	isTouching: bool,
	boost: i32,
	team: bool,
}

/* KINEMATICS AND BALL PREDICTION */
#[derive(Debug)]
struct Kinematics {
	position: VectorC,
	velocity: VectorC,
	acceleration: VectorC,
	time: f32,
	baseUnitName: String,
}

impl Kinematics {
	fn printer(self, table: bool) {
		if table == true {
			println!("             -------------------------------------");
			println!("Position     | {:.5} | {:.5} | {:.5} |", self.position.x, self.position.y, self.position.z);
			println!("             -------------------------------------");
			println!("Velocity     | {:.5} | {:.5} | {:.5} |", self.velocity.x, self.velocity.y, self.velocity.z);
			println!("             -------------------------------------");
			println!("Acceleration | {:.5} | {:.5} | {:.5} |", self.acceleration.x, self.acceleration.y, self.acceleration.z);
			println!("             -------------------------------------");
		} else {
			println!("At time {}, the object will be {} {}s away going at a speed of {} {}s per second.", self.time, self.position.magnitude(), self.baseUnitName, self.velocity.magnitude(), self.baseUnitName);
		}
	}
	fn inseconds(self, timef: f32) -> Kinematics {
		let timeu = timef - self.time;
		Kinematics {
			position: VectorC {
				x: ((self.velocity.x * timeu) + (0.5 * self.acceleration.x * timeu * timeu)),
				y: ((self.velocity.y * timeu) + (0.5 * self.acceleration.y * timeu * timeu)),
				z: ((self.velocity.z * timeu) + (0.5 * self.acceleration.z * timeu * timeu)),
			},
			velocity: VectorC {
				x: self.velocity.x + (self.acceleration.x * timeu),
				y: self.velocity.y + (self.acceleration.y * timeu),
				z: self.velocity.z + (self.acceleration.z * timeu),
			},
			acceleration: self.acceleration,
			time: timeu,
			baseUnitName: "Unit".to_string(),
		}
	}

	fn inminutes(self, timefm: f32) -> Kinematics {
		let timef = timefm / 60.0;
		let timeu = timef - self.time;
		Kinematics {
			position: VectorC {
				x: ((self.velocity.x * timeu) + (0.5 * self.acceleration.x * timeu * timeu)),
				y: ((self.velocity.y * timeu) + (0.5 * self.acceleration.y * timeu * timeu)),
				z: ((self.velocity.z * timeu) + (0.5 * self.acceleration.z * timeu * timeu)),
			},
			velocity: VectorC {
				x: self.velocity.x + (self.acceleration.x * timeu),
				y: self.velocity.y + (self.acceleration.y * timeu),
				z: self.velocity.z + (self.acceleration.z * timeu),
			},
			acceleration: self.acceleration,
			time: timeu,
			baseUnitName: "Unit".to_string(),
		}
	}

	fn inhours(self, timefh: f32) -> Kinematics {
		let timefm = timefh / 60.0;
		let timef = timefm / 60.0;
		let timeu = timef - self.time;
		Kinematics {
			position: VectorC {
				x: ((self.velocity.x * timeu) + (0.5 * self.acceleration.x * timeu * timeu)),
				y: ((self.velocity.y * timeu) + (0.5 * self.acceleration.y * timeu * timeu)),
				z: ((self.velocity.z * timeu) + (0.5 * self.acceleration.z * timeu * timeu)),
			},
			velocity: VectorC {
				x: self.velocity.x + (self.acceleration.x * timeu),
				y: self.velocity.y + (self.acceleration.y * timeu),
				z: self.velocity.z + (self.acceleration.z * timeu),
			},
			acceleration: self.acceleration,
			time: timeu,
			baseUnitName: "Unit".to_string(),
		}
	}

	fn indays(self, timefd: f32) -> Kinematics {
		let timefh = timefd / 12.0;
		let timefm = timefh / 60.0;
		let timef = timefm / 60.0;
		let timeu = timef - self.time;
		Kinematics {
			position: VectorC {
				x: ((self.velocity.x * timeu) + (0.5 * self.acceleration.x * timeu * timeu)),
				y: ((self.velocity.y * timeu) + (0.5 * self.acceleration.y * timeu * timeu)),
				z: ((self.velocity.z * timeu) + (0.5 * self.acceleration.z * timeu * timeu)),
			},
			velocity: VectorC {
				x: self.velocity.x + (self.acceleration.x * timeu),
				y: self.velocity.y + (self.acceleration.y * timeu),
				z: self.velocity.z + (self.acceleration.z * timeu),
			},
			acceleration: self.acceleration,
			time: timeu,
			baseUnitName: "Unit".to_string(),
		}
	}
}

/*   CUSTOM VECTOR   */
#[derive(Debug, Copy, Clone)]
struct VectorC {
	x: f32,
	y: f32,
	z: f32,
}
impl VectorC {
	fn magnitude(&self) -> f32 {
		let rsqu: f32 = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
		rsqu.sqrt()
	}
}
impl Add for VectorC {
	type Output = Self;
	fn add(self, vec2: Self) -> Self {
		VectorC {x: self.x + vec2.x, y: self.y + vec2.y, z: self.z + vec2.z}
	}
}
impl Sub for VectorC {
	type Output = Self;
	fn sub(self, vec2: Self) -> Self {
		VectorC {x: self.x - vec2.x, y: self.y - vec2.y, z: self.z - vec2.z}
	}
}
impl Mul for VectorC {
	type Output = Self;
	fn mul(self, vec2: Self) -> Self {
		VectorC {x: self.x * vec2.x, y: self.y * vec2.y, z: self.z * vec2.z}
	}
}
impl Div for VectorC {
	type Output = Self;
	fn div(self, vec2: Self) -> Self {
		VectorC {x: self.x / vec2.x, y: self.y / vec2.y, z: self.z / vec2.z}
	}
}

/*   CUSTOM ROTATOR   */
#[derive(Debug, Copy, Clone)]
struct RotatorC {
	pitch: f32,
	yaw: f32,
	roll: f32,
}
impl RotatorC {
	fn magnitude(&self) -> f32 {
		let rsqu: f32 = self.pitch.powf(2.0) + self.yaw.powf(2.0) + self.roll.powf(2.0);
		rsqu.sqrt()
	}
}
impl Add for RotatorC {
	type Output = Self;
	fn add(self, rot2: Self) -> Self {
		RotatorC {pitch: self.pitch + rot2.pitch, yaw: self.yaw + rot2.yaw, roll: self.roll + rot2.roll}
	}
}
impl Sub for RotatorC {
	type Output = Self;
	fn sub(self, rot2: Self) -> Self {
		RotatorC {pitch: self.pitch - rot2.pitch, yaw: self.yaw - rot2.yaw, roll: self.roll - rot2.roll}
	}
}
impl Mul for RotatorC {
	type Output = Self;
	fn mul(self, rot2: Self) -> Self {
		RotatorC {pitch: self.pitch * rot2.pitch, yaw: self.yaw * rot2.yaw, roll: self.roll * rot2.roll}
	}
}
impl Div for RotatorC {
	type Output = Self;
	fn div(self, rot2: Self) -> Self {
		RotatorC {pitch: self.pitch / rot2.pitch, yaw: self.yaw / rot2.yaw, roll: self.roll / rot2.roll}
	}
}