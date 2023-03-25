//! This module implements the NIC structure, representing an e1000-compatible NIC.

use kernel::device::bar::BAR;
use kernel::device::manager::PhysicalDevice;
use kernel::errno::Errno;
use kernel::net::BindAddress;
use kernel::net::MAC;
use kernel::net;

/// Register address: EEPROM/Flash Control & Data
const REG_EECD: u16 = 0x10;
/// Register address: EEPROM Read Register
const REG_EERD: u16 = 0x14;

/// Transmit descriptor command flag: End of Packet
const TX_CMD_EOP: u8 = 0x01;
/// Transmit descriptor command flag: Insertion of FCS
const TX_CMD_IFCS: u8 = 0x02;
/// Transmit descriptor command flag: Insert checksum
const TX_CMD_IC: u8 = 0x04;
/// Transmit descriptor command flag: Report status
const TX_CMD_RS: u8 = 0x08;
/// Transmit descriptor command flag: Report Packet Sent
const TX_CMD_RPS: u8 = 0x10;
/// Transmit descriptor command flag: VLAN Packet Enable
const TX_CMD_VLE: u8 = 0x40;
/// Transmit descriptor command flag: Interrupt Delay Enable
const TX_CMD_IDE: u8 = 0x80;

/// The receive descriptor.
#[repr(packed)]
struct RXDesc {
	/// The physical address of the data.
	addr: u64,
	/// The length of the data.
	length: u16,
	/// The packet's checksum.
	checksum: u16,
	/// Status flags.
	status: u8,
	/// Error flags.
	errors: u8,
	/// TODO doc
	special: u16,
}

// TODO: This is the legacy structure. Add support for the new version
/// The transmit descriptor.
#[repr(packed)]
struct TXDesc {
	/// The physical address of the data.
	addr: u64,
	/// The length of the data.
	length: u16,
	/// CheckSum Offset: the offset at which the checksum is to be placed in the given data.
	cso: u8,
	/// Command flags.
	cmd: u8,
	/// Status flags.
	status: u8,
	/// CheckSum Start: the offset at which computation of the checksum starts in the given data.
	css: u8,
	/// TODO doc
	special: u16,
}

/// Structure representing a Network Interface Card.
pub struct NIC {
	/// TODO doc
	status_reg: u16,
	/// TODO doc
	command_reg: u16,

	/// The BAR0 of the device.
	bar0: BAR,

	/// Tells whether the EEPROM exist.
	eeprom_exists: bool,

	/// The NIC's mac address.
	mac: [u8; 6],
}

impl NIC {
	/// Creates a new instance using the given device.
	pub fn new(dev: &dyn PhysicalDevice) -> Result<Self, &str> {
		let status_reg = dev.get_status_reg().ok_or("Invalid PCI informations for NIC!")?;
		let command_reg = dev.get_command_reg().ok_or("Invalid PCI informations for NIC!")?;

		let bar0 = dev.get_bars()[0].clone().ok_or("Invalid BAR for NIC!")?;

		let mut n = Self {
			status_reg,
			command_reg,

			bar0,

			eeprom_exists: false,

			mac: [0; 6],
		};
		n.detect_eeprom();
		n.read_mac();
		n.init_desc();

		Ok(n)
	}

	/// Sends a command to read at address `addr` in the NIC memory.
	fn read_command(&self, addr: u16) -> u32 {
		self.bar0.read::<u32>(addr as _) as _
	}

	/// Sends a command to write the value `val` at address `addr` in the NIC memory.
	fn write_command(&self, addr: u16, val: u32) {
		self.bar0.write::<u32>(addr as _, val as _);
	}

	/// Detects whether the EEPROM exists.
	fn detect_eeprom(&mut self) {
		self.eeprom_exists = self.read_command(REG_EECD) & (1 << 8) != 0;
	}

	/// Reads from the EEPROM at address `addr`.
	fn eeprom_read(&self, addr: u8) -> u32 {
		// Acquire EEPROM
		self.write_command(REG_EECD, self.read_command(REG_EECD) | (1 << 6));

		// Specify read address
		self.write_command(REG_EERD, 1 | ((addr as u32) << 8));

		let data = if self.eeprom_exists {
			loop {
				let val = self.read_command(REG_EERD);
				if val & (1 << 4) != 0 {
					break (val >> 16) & 0xffff;
				}
			}
		} else {
			// TODO
			todo!();
		};

		// Release EEPROM
		self.write_command(REG_EECD, self.read_command(REG_EECD) & !(1 << 6));

		data
	}

	/// Reads the MAC address from the NIC's EEPROM.
	fn read_mac(&mut self) {
		let val = self.eeprom_read(0);
		self.mac[0] = (val & 0xff) as u8;
		self.mac[1] = ((val >> 8) & 0xff) as u8;

		let val = self.eeprom_read(1);
		self.mac[2] = (val & 0xff) as u8;
		self.mac[3] = ((val >> 8) & 0xff) as u8;

		let val = self.eeprom_read(2);
		self.mac[4] = (val & 0xff) as u8;
		self.mac[5] = ((val >> 8) & 0xff) as u8;
	}

	/// Initializes transmit and receive descriptors.
	fn init_desc(&self) {
		// TODO
		todo!();
	}

	/// Receives data using the given descriptor.
	fn receive(&self, _rx_desc: &mut RXDesc) {
		// TODO
		todo!();
	}

	/// Transmits the data of the given descriptor.
	fn transmit(&self, _tx_desc: &mut TXDesc) {
		// TODO
		todo!();
	}
}

impl net::Interface for NIC {
	fn get_name(&self) -> &[u8] {
		// TODO replace "TODO" with interface ID
		b"ethTODO"
	}

	fn is_up(&self) -> bool {
		// TODO
		todo!();
	}

	fn get_mac(&self) -> &MAC {
		&self.mac
	}

	fn get_addresses(&self) -> &[BindAddress] {
		// TODO
		todo!();
	}

	fn read(&mut self, _buff: &mut [u8]) -> Result<(u64, bool), Errno> {
		// TODO
		todo!();
	}

	fn write(&mut self, _buff: &[u8]) -> Result<u64, Errno> {
		// TODO do asynchronously
		/*let mut i = 0;

		while i < buff.len() {
			let desc = &mut self.tx_descs[self.curr_tx_desc];
			desc.addr = buff.as_ptr() as _;
			desc.length = min(buff.len() - i, u16::MAX as usize) as _;
			desc.cmd = ; // TODO
			desc.status = 0;

			let next_desc = (self.curr_tx_desc + 1) % TX_DESC_COUNT;
			self.write_command(, next_desc);

			// TODO wait until status is not zero
		}

		Ok(i as _)*/
		todo!();
	}
}
