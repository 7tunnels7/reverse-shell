# Reverse SSH

Reverse-Shell is a lightweight Rust-based dummy SSH server for testing and development purposes. It allows you to simulate SSH connections, supports authentication, and enables data forwarding to a remote server. This tool is perfect for smooth SSH development and testing workflows.

## Features

- Simulate SSH connections
- Support for SSH authentication
- Data forwarding to a remote server

## Getting Started

To get started with SSHForwarder, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/ssh-forwarder.git`
2. Build the project: `cargo build`
3. Run the server: `cargo run`

### Usage

1. Modify the code to set your desired configuration options:
   - `remote_host`: The IP address or hostname of the remote server.
   - `remote_port`: The SSH port on the remote server.
   - `remote_user`: Your username on the remote server.
   - `local_host`: The local hostname or IP address on which to listen for incoming connections.
   - `local_port`: The local port to listen on.
   - `remote_port_forwarded`: The remote port to forward incoming connections to.
   
4. Open a new terminal window and establish a reverse SSH tunnel by running the following command:
   ```shell
   ssh -R <remote-port>:localhost:<local-port> <remote-user>@<remote-host>
   ```
   Replace `<remote-port>` with the port number on the remote server where you want to forward incoming connections. Replace `<local-port>` with the local port specified in the code. Additionally, replace `<remote-user>` with your username on the remote server, and `<remote-host>` with the hostname or IP address of the remote server.

   Example command:
   ```shell
   ssh -R 9000:localhost:8081 admin@203.0.113.10
   ```

5. Incoming connections to the remote port specified in the SSH command will now be forwarded to the local port specified in the code.

**Note:** Ensure that SSH server configuration on the remote server allows for remote port forwarding (`GatewayPorts` option set to `yes` in `sshd_config` file).
## Configuration

SSHForwarder can be configured by modifying the `config.toml` file. Customize authentication settings, port numbers, and more according to your requirements.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it as per the terms of the license.

## Contributions

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## Depn

[ssh2](https://github.com/alexcrichton/ssh2-rs) (0.8)

## Disclaimer

SSHForwarder is intended for testing and development purposes only. Use it responsibly and ensure compliance with all applicable laws and regulations.

🚀 Happy SSH testing and development! 🚀
