use std::ops::{Mul, Add, Sub, Div};
use std::io;

fn main() {
	unimplemented!();
}

fn packetConverter(arg: Type) -> RetType {
	unimplemented!();
}

fn setup(packet: Packet) -> Controller {
	let prediction = Kinematics {
		position: packet.ballLocation,
		velocity: packet.ballVelocity,
		acceleration: VectorC {x: 0.0, y: 0.0, z: -650.0},
		time: 0.0
		baseUnitName: "Unit".to_string(),
	};
	
	// Needs Controller limiter before return. -1:1
}

#[derive(Debug)]
enum CurrentState {
	kickoff,
	attb,
}

trait StatePicker {
	fn available(pack: Packet) -> bool;
	fn execute(pack: Packet) -> Controller;
	fn expired(pack: Packet) -> bool;
}

#[derive(Debug)]
struct kickoff();
impl StatePicker for kickoff {
	fn available(pack: Packet) -> bool {
		if pack.roundActive == true {
			true
		}
		false
	}
	fn execute(pack: Packet) -> Controller {
		let mut controllerBoost = false;
		let mut controllerSteer = 0.0;
		let mut controllerJump = false;
		let mut controllerPitch = 0.0;
		let mut controllerYaw = 0.0;
		let mut controllerRoll = 0.0;
		controllerState = Controller {
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
	fn expired(pack: Packet) -> bool {
		if pack.
	}
}

#[derive(Debug)]
struct attb();
impl StatePicker for attb {
	fn available(pack: Packet) -> bool {
		true
	}
	fn execute(pack: Packet) -> Controller {
		let mut controllerSteer = 0.0; // Needs steering mechanism.
		controllerState = Controller {
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
		if pack.ballLocation.x == 0 && pack.ballLocation.y == 0 {
		 	false
		}
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
		let timeu = timef - self.time
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
		}
	}

	fn inminutes(self, timefm: f64) -> Kinematics {
		let timef = timefm / 60.0;
		let timeu = timef - self.time
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
		}
	}

	fn inhours(self, timefh: f64) -> Kinematics {
		let timefm = timefh / 60.0;
		let timef = timefm / 60.0;
		let timeu = timef - self.time
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
		}
	}

	fn indays(self, timefd: f64) -> Kinematics {
		let timefh = timefd / 12.0;
		let timefm = timefh / 60.0;
		let timef = timefm / 60.0;
		let timeu = timef - self.time
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