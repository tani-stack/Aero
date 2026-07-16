#![no_std]
use core::fmt::Debug;

/// ============ NETWORK STACK BASICS ============
/// Lightweight networking for vehicle communication

/// ============ MESSAGE TYPES ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    // Telemetry
    Telemetry,
    
    // Commands
    MotorCommand,
    SteeringCommand,
    ArmCommand,
    DisarmCommand,
    
    // Status
    StatusReport,
    HealthReport,
    
    // Diagnostics
    DiagnosticRequest,
    DiagnosticResponse,
}

/// ============ COMMUNICATION PROTOCOLS ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Mavlink,        // Standard for drones/robots
    Can,            // Standard for cars
    Uart,           // Generic serial
    I2c,            // Short-distance
    Spi,            // High-speed local
    Ethernet,       // Network (future)
}

/// ============ PACKET STRUCTURE ============
#[derive(Debug, Clone, Copy)]
pub struct Packet {
    pub source_id: u8,
    pub dest_id: u8,
    pub packet_type: MessageType,
    pub protocol: Protocol,
    pub payload_len: u16,
    pub sequence: u16,
    pub timestamp_ms: u32,
    pub crc: u16,
}

impl Packet {
    pub fn new(src: u8, dest: u8, msg_type: MessageType, proto: Protocol) -> Self {
        Self {
            source_id: src,
            dest_id: dest,
            packet_type: msg_type,
            protocol: proto,
            payload_len: 0,
            sequence: 0,
            timestamp_ms: 0,
            crc: 0,
        }
    }

    pub fn compute_crc(&self) -> u16 {
        let mut crc: u16 = 0xFFFF;
        let data = [
            self.source_id,
            self.dest_id,
            self.packet_type as u8,
            (self.payload_len >> 8) as u8,
            (self.payload_len & 0xFF) as u8,
        ];
        for byte in &data {
            crc = crc_xmodem(crc, *byte);
        }
        crc
    }
}

fn crc_xmodem(mut crc: u16, byte: u8) -> u16 {
    crc ^= (byte as u16) << 8;
    for _ in 0..8 {
        crc <<= 1;
        if crc & 0x10000 != 0 {
            crc ^= 0x1021;
        }
    }
    crc
}

/// ============ MESSAGE QUEUE ============
pub struct MessageQueue {
    messages: [Option<Packet>; 128],
    head: u8,
    tail: u8,
    count: u8,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            messages: [None; 128],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, packet: Packet) -> Result<(), ()> {
        if self.count >= 128 {
            return Err(());
        }
        self.messages[self.tail as usize] = Some(packet);
        self.tail = self.tail.wrapping_add(1);
        self.count += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<Packet> {
        if self.count == 0 {
            return None;
        }
        let msg = self.messages[self.head as usize];
        self.head = self.head.wrapping_add(1);
        self.count -= 1;
        msg
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

/// ============ COMMUNICATION MANAGER ============
pub struct CommManager {
    tx_queue: MessageQueue,
    rx_queue: MessageQueue,
    vehicle_id: u8,
    ground_station_id: u8,
    messages_sent: u32,
    messages_received: u32,
}

impl CommManager {
    pub fn new(vehicle_id: u8) -> Self {
        Self {
            tx_queue: MessageQueue::new(),
            rx_queue: MessageQueue::new(),
            vehicle_id,
            ground_station_id: 255,
            messages_sent: 0,
            messages_received: 0,
        }
    }

    pub fn send(&mut self, packet: Packet) -> Result<(), ()> {
        self.tx_queue.enqueue(packet)?;
        self.messages_sent += 1;
        Ok(())
    }

    pub fn receive(&mut self) -> Option<Packet> {
        self.rx_queue.dequeue()
    }

    pub fn process_incoming(&mut self, packet: Packet) -> Result<(), ()> {
        self.rx_queue.enqueue(packet)?;
        self.messages_received += 1;
        Ok(())
    }
}
