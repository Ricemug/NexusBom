// PLM/ERP adapters placeholder
// TODO: Implement SAP and Oracle adapters

#[cfg(feature = "sap")]
pub mod sap;

#[cfg(feature = "oracle")]
pub mod oracle;

pub mod rest;
