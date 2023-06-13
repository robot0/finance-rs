# FinanceHub: Your Financial Toolset Backend

FinanceHub is a backend crafted meticulously in Rust, aimed at serving as a backbone for financial Software as a Service (SaaS) products. While it currently boasts a powerful currency conversion API, the vision is to continually expand this repository with a wide array of financial tools and services.

## Features

### Current Features

- **Currency Conversion API**: Convert between different currencies with ease and accuracy.

### Planned Features

- Additional financial analysis tools
- Integration with various financial data providers
- Enhanced security features
- Comprehensive API documentation
- User authentication and authorization
- ...and more!

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your local machine.
- Basic knowledge of Rust programming language.
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) installed on your local machine.

### Installation

1. Clone the repository.

   ```bash
   git clone https://github.com/yourusername/FinanceHub.git
   ```

2. Navigate to the project directory.

   ```bash
   cd FinanceHub
   ```

3. Run the project.

   ```bash
   cargo run
   ```

## Usage

### Currency Conversion API

To convert currencies, make a request to the API endpoint:

```bash
GET /api/convert?from=USD&to=EUR&amount=100
```

This will convert 100 USD to EUR.

Parameters:

- `from`: The currency code of the currency you want to convert from.
- `to`: The currency code of the currency you want to convert to.
- `amount`: The amount of currency you want to convert.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Your Name - [kenbabatope@gmail.com](mailto:kenbabatope@gmail.com)

Project Link: [https://github.com/robot0/finance-rs](https://github.com/robot0/finance-rs)
