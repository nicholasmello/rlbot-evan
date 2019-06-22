use std::ops::{Mul, Add, Sub, Div};
use std::{error::Error, f32::consts::PI};

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

fn get_input(
    player_index: usize,
    packet: &rlbot::GameTickPacket,
) -> Option<rlbot::ControllerState> {
    let mycontrollerstate = packetConverter();

    Some(rlbot::ControllerState {
        throttle: mycontrollerstate.throttle,
        steer: mycontrollerstate.steer,
        pitch: mycontrollerstate.pitch,
        yaw: mycontrollerstate.yaw,
        roll: mycontrollerstate.roll,
        jump: mycontrollerstate.jump,
        boost: mycontrollerstate.boost,
        handbreak: mycontrollerstate.drift,
    })
}

fn packetConverter() -> Controller {
	let mut rawPacket = &rlbot::GameTickPacket
	let mut mypacket = Packet {
		ballLocation: VectorC {x: y: z: },
		ballVelocity: VectorC {x: y: z: },
		roundActive: bool,
		evan: PacketPlayer {
			location: VectorC {x: y: z: },
			velocity: VectorC {x: y: z: },
			rotation: VectorC {x: y: z: },
			isTouching: bool,
			boost: i8,
			Team: bool,
		},
		opponent: PacketPlayer {
			location: VectorC {x: y: z: },
			velocity: VectorC {x: y: z: },
			rotation: VectorC {x: y: z: },
			isTouching: bool,
			boost: i8,
			Team: bool,
		},
	}
	evan_input(mypacket)
}

fn evan_input(packet: Packet) -> Controller {
	let prediction = Kinematics {
		position: packet.ballLocation,
		velocity: packet.ballVelocity,
		acceleration: VectorC {x: 0.0, y: 0.0, z: -650.0},
		time: 0.0,
		baseUnitName: "Unit".to_string(),
	};

	// Needs Controller limiter before return. -1:1
}

#[derive(Debug)]
enum CurrentState {
	kickoff,
	attb,
}

#[derive(Debug)]
struct kickoff {
	active: bool
}
impl kickoff {
	fn available(&self, pack: Packet) -> bool {
		if pack.roundActive == true {
			return true;
		}
		false
	}
	fn execute(&self, pack: Packet) -> Controller {
		let mut controllerBoost = false;
		let mut controllerSteer = 0.0;
		let mut controllerJump = false;
		let mut controllerPitch = 0.0;
		let mut controllerYaw = 0.0;
		let mut controllerRoll = 0.0;
		Controller {
			throttle: 1.0,
			boost: controllerBoost,
			steer: controllerSteer,
			jump: controllerJump,
			pitch: controllerPitch,
			yaw: controllerYaw,
			roll: controllerRoll,
			drift: false,
		}
	}
	fn expired(&self, pack: Packet) -> bool {
		if pack.ballLocation.x == 0.0 && pack.ballLocation.y == 0.0 {
		 	return false;
		}
		true
	}
}

#[derive(Debug)]
struct attb();
impl attb {
	fn available(pack: Packet) -> bool {
		true
	}
	fn execute(pack: Packet) -> Controller {
		let mut controllerSteer = 0.0; // Needs steering mechanism.
		Controller {
			throttle: 1.0,
			boost: false,
			steer: controllerSteer,
			jump: false,
			pitch: 0.0,
			yaw: 0.0,
			roll: 0.0,
			drift: false,
		}
	}
	fn expired(pack: Packet) -> bool {
		true
	}
}

/* CUSTOM CONTROLLER AND PACKET */
#[derive(Debug)]
struct Controller {
	throttle: f64,
	boost: bool,
	steer: f64,
	jump: bool,
	pitch: f64,
	yaw: f64,
	roll: f64,
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
	rotation: VectorC,
	isTouching: bool,
	Boost: i8,
	Team: bool,
}

/* KINEMATICS AND BALL PREDICTION */
#[derive(Debug)]
struct Kinematics {
	position: VectorC,
	velocity: VectorC,
	acceleration: VectorC,
	time: f64,
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
	fn inseconds(self, timef: f64) -> Kinematics {
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

	fn inminutes(self, timefm: f64) -> Kinematics {
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

	fn inhours(self, timefh: f64) -> Kinematics {
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

	fn indays(self, timefd: f64) -> Kinematics {
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
	x: f64,
	y: f64,
	z: f64,
}
impl VectorC {
	fn magnitude(&self) -> f64 {
		let rsqu: f64 = self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0);
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