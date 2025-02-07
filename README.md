# QuietMap

QuietMap is a covert tool designed for automated network mapping, packet collection, and server information retrieval. It runs silently and gathers essential data about a target, all while avoiding detection. The project is written in Rust and currently supports the use of **Nmap** for network scanning. **Wireshark** and the **Shodan API** will be integrated in the future for packet analysis and server information collection.

## Features

- **Network Mapping (Nmap)**: Quietly maps open ports and services of a target network at random intervals.
- **Covert Operations**: The tool is designed to work in a way that minimizes detection risks by varying scan intervals and timing.
- **Data Storage**: Scan results are saved in text files categorized by scan type for easy reference.
- **Future Integrations**: Planned integrations with **Wireshark** for packet analysis and **Shodan API** for gathering server information.

## Requirements

- **Rust**: The project is written in Rust. Make sure you have the latest version installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- **Nmap**: For the network scanning feature, you will need **Nmap** installed on your system. You can install it from [nmap.org](https://nmap.org/).

## Installation

1. Clone the repository:
  
  ```bash
  git clone https://github.com/AustinSAdams/QuietMap.git
  ```
  
2. Navigate to the project directory:
  
  ```bash
  cd QuietMap
  ```
  
3. Build the project:
  
  ```bash
  cargo build --release
  ```
  
4. Run the tool:
  
  ```bash
  cargo run
  ```
  

## Usage

### Nmap Feature

To run the Nmap feature, simply provide the target IP or domain name:

```bash
cargo run
```

This will initiate a silent scan and store the results in a text file categorized by scan type.

### Future Features

Once Wireshark and Shodan integrations are added, new commands will be available for packet capture and server information retrieval. Stay tuned for updates!

## Contributing

Feel free to open issues and pull requests. Contributions to improve the stealth and functionality of the tool are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.