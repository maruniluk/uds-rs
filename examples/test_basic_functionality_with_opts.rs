//!
//! To run the example, setup vcan0 using following commands:
//! ```bash
//! sudo modprobe can
//! sudo ip link add dev vcan0 type vcan
//! sudo ip link set up vcan0
//! ```
//!
use bitflags::Flags;
use env_logger;
use log::error;
use uds_rs::{ResetType, UdsClient, UdsError};

fn create_socket() -> uds_rs::UdsSocket {
    let mut behav = tokio_socketcan_isotp::IsoTpBehaviour::CAN_ISOTP_RX_PADDING;
    behav.set(
        tokio_socketcan_isotp::IsoTpBehaviour::CAN_ISOTP_RX_PADDING,
        true,
    );

    let tp_options = tokio_socketcan_isotp::IsoTpOptions::new(
        behav,
        std::time::Duration::from_secs(1),
        u8::MAX,
        0x55,
        0xAA,
        u8::MAX,
    );

    let mut options = None;

    match tp_options {
        Ok(mut o) => {
            options = Some(o);
        }
        Err(_) => println!("cannot"),
    }
    let tp_socket = uds_rs::UdsSocket::new_with_opts(
        "can0",
        tokio_socketcan_isotp::StandardId::new(0x0A).unwrap(),
        tokio_socketcan_isotp::StandardId::new(0x74).unwrap(),
        options,
        None,
        None,
    )
    .unwrap();
    return tp_socket;
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), UdsError> {
    env_logger::init();
    let c = UdsClient::new_from_socket(create_socket());

    let read_data_result = c.read_data_by_identifier(&[0xf197]).await;

    match read_data_result {
        Ok(x) => println!("Read data by identifier received {:#x?}", x),
        Err(e) => error!(
            "Read single data by identifier failed with error: {:#x?}",
            e
        ),
    };
    let read_dtc_information = c.report_dtc_by_status_mask(0xff).await;

    match read_dtc_information {
        Ok(x) => println!("Read dtc by status mask: {:#x?}", x),
        Err(e) => error!("{} Read dtc by status mask failed with error: {:#x?}", e, e),
    }

    let clear_dtc_information = c.clear_diagnostic_information(0xffffff).await;

    match clear_dtc_information {
        Ok(x) => println!("{:#x?}", x),
        Err(e) => error!("Clear diagnostic information failed with error: {:#x?}", e),
    };
    let ecu_reset_result = c.ecu_reset(ResetType::KeyOffOnReset).await;

    match ecu_reset_result {
        Ok(x) => println!("{:#x?}", x),
        Err(e) => error!("Ecu reset failed with error: {:#x?}", e),
    };
    Ok(())
}
