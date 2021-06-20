//! # embedded-storage - A Storage Abstraction Layer for Embedded Systems
//!
//! Storage traits to allow on and off board storage devices to read and write
//! data.

#![doc(html_root_url = "https://docs.rs/embedded-storage/0.1.0")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Currently contains [`OverlapIterator`]
pub mod iter;
/// Technology specific traits for NOR Flashes
pub mod nor_flash;

/// A region denotes a contiguous piece of memory between two addresses.
pub trait Region {
	/// Check if `address` is contained in the region of `Self`
	fn contains(&self, address: u32) -> bool;
}

/// Transparent read only storage trait
pub trait ReadStorage {
	/// An enumeration of storage errors
	type Error;

	/// Read a slice of data from the storage peripheral, starting the read
	/// operation at the given address offset, and reading `bytes.len()` bytes.
	///
	/// This should throw an error in case `bytes.len()` will be larger than
	/// `self.capacity() - offset`.
	fn try_read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error>;

	/// The capacity of the storage peripheral in bytes.
	fn capacity(&self) -> usize;
}

/// Transparent read/write storage trait
pub trait Storage: ReadStorage {
	/// Write a slice of data to the storage peripheral, starting the write
	/// operation at the given address offset (between 0 and `self.capacity()`).
	///
	/// **NOTE:**
	/// This function will automatically erase any pages necessary to write the given data,
	/// and might as such do RMW operations at an undesirable performance impact.
	fn try_write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error>;
}

/// Storage that supports erasing of data pages
pub trait ErasableStorage {
	/// An enumeration of storage erase errors
	type Error;

	/// The minimum number of bytes the storage peripheral can erase
	const ERASE_SIZE: u32;

	/// Erase the given storage range, clearing all data within `[from..to]`.
	/// The given range will contain all 1s afterwards.
	///
	/// This should return an error if the range is not aligned to a proper
	/// erase resolution
	/// If power is lost during erase, contents of the page are undefined.
	/// `from` and `to` must both be multiples of `ERASE_SIZE` and `from` <= `to`.
	fn try_erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error>;
}
